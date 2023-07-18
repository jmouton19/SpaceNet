// Establish a connection to the server

const source = new EventSource('http://127.0.0.1:3030/api');

const trans_time=500;

let node_polygons={};
let node_players={};

window.addEventListener("resize", updateSvgSize);

const colorScale = d3.scaleOrdinal(d3.schemeCategory10);

const svg = d3.select("#svg-container")
    .append("svg")
    .attr("id", "svg")
    .attr("width", Math.min(window.innerWidth, window.innerHeight) * 0.8) // Set the width as 80% of the smaller dimension
    .attr("height", Math.min(window.innerWidth, window.innerHeight) * 0.8); // Set the height as 80% of the smaller dimension

// Function to update the SVG container size when the window is resized

function updateSvgSize() {
    const containerSize = Math.min(window.innerWidth, window.innerHeight) * 0.8;
    svg.attr("width", containerSize);
    svg.attr("height", containerSize);
    Object.keys(node_polygons).forEach(sender_id => {
        updatePolygon(sender_id);
    });
}

function scalePoint(point) {
    const scaledX = (point[0] / 100) * svg.attr("width");
    const scaledY = (point[1] / 100) * svg.attr("height");
    return [scaledX, scaledY];
}

function updatePolygon(sender_id) {
    const node = node_polygons[sender_id];
    if (node) {
        const polygon = node.polygon.map(scalePoint);
        const site = scalePoint(node.site);
        const color = colorScale(sender_id);
        const strokeWidth = svg.attr("width") * 0.005; // 1% of the container width
        const circleRadius = svg.attr("width") * 0.004;

        // Select the existing polygon elements with the specified sender_id
        const existingPolygons = svg.selectAll(`polygon[data-sender-id="${sender_id}"]`);
        const existingCircles = svg.selectAll(`circle[data-sender-id="${sender_id}"]`);

        // Check if there are existing polygons
        if (existingPolygons.size() > 0) {
            // Transition the existing polygons to the new polygon coordinates and color
            existingPolygons
                .transition()
                .duration(trans_time) // Set the duration of the transition in milliseconds
                .attr("points", polygon.map(point => point.join(",")).join(" "))
                .attr("fill", color);

            // Update the data attribute of the existing polygons
           // existingPolygons.attr("data-sender-id", sender_id);

            existingCircles
                .transition()
                .duration(trans_time) // Set the duration of the transition in milliseconds
                .attr("cx", site[0])
                .attr("cy", site[1]);

            // Update the data attribute of the existing circles
            //existingCircles.attr("data-sender-id", sender_id);
        } else {
            // No existing polygons, create new ones
            svg.append("polygon")
                .attr("points", polygon.map(point => point.join(",")).join(" "))
                .attr("fill", color)
                .attr("stroke", "black")
                .attr("stroke-width", strokeWidth)
                .attr("data-sender-id", sender_id);

            svg.append("circle")
                .attr("cx", site[0])
                .attr("cy", site[1])
                .attr("r", circleRadius)
                .attr("fill", "black")
                .attr("data-sender-id", sender_id);
        }

        const circles = svg.selectAll("circle");
        circles.each(function() {
            this.parentNode.appendChild(this); // Move the circle to the end of its parent
        });

        const texts = svg.selectAll("text");
        texts.each(function() {
            this.parentNode.appendChild(this); // Move the circle to the end of its parent
        });

    }
}


function removePolygon(sender_id) {
    // Select and remove the existing polygon and circle elements with the specified sender_id
    const existingPolygons = svg.selectAll(`polygon[data-sender-id="${sender_id}"]`);
    const existingCircles = svg.selectAll(`circle[data-sender-id="${sender_id}"]`);

    // Apply transition to fade out and remove the existing polygons
    existingPolygons
        .transition()
        .duration(trans_time) // Set the duration of the transition in milliseconds
        .style("opacity", 0)
        .remove();

    // Apply transition to fade out and remove the existing circles
    existingCircles
        .transition()
        .duration(trans_time) // Set the duration of the transition in milliseconds
        .style("opacity", 0)
        .remove();

    // Remove the corresponding entry from the node_polygons map
    delete node_polygons[sender_id];

    //delete node_polygons[sender_id];
}
function updatePlayer(player_id) {
    const player = node_players[player_id];
    if (player) {
        const site = scalePoint([player.x,player.y]);
        const color = colorScale(player.sender_id+player.sender_id);
        const circleRadius = svg.attr("width") * 0.004;
        const textOffset = circleRadius * 1.5;
        const fontSize = svg.attr("width") * 0.02;

        // Select the existing polygon elements with the specified sender_id
        const existingCircles = svg.selectAll(`circle[data-player-id="${player_id}"]`);
        const existingTexts = svg.selectAll(`text[data-player-id="${player_id}"]`);


        // Check if there are existing polygons
        if (existingCircles.size() > 0) {

            existingCircles
                .transition()
                .duration(trans_time) // Set the duration of the transition in milliseconds
                .attr("cx", site[0])
                .attr("cy", site[1]);

            existingTexts
                .transition()
                .duration(trans_time)
                .attr("x", site[0] + textOffset)
                .attr("y", site[1]);

            // Update the data attribute of the existing circles
            //existingCircles.attr("data-player-id", player_id);
        } else {
            svg.append("circle")
                .attr("cx", site[0])
                .attr("cy", site[1])
                .attr("r", circleRadius)
                .attr("fill", color)
                .attr("data-player-id", player_id);

            svg.append("text")
                .attr("x", site[0] + textOffset)
                .attr("y", site[1])
                .attr("data-player-id", player_id)
                .text(player_id)
                .style("font-size", `${fontSize}px`);
        }

    }
}

function removePlayer(player_id) {
    const existingCircles = svg.selectAll(`circle[data-player-id="${player_id}"]`);
    const existingTexts = svg.selectAll(`text[data-player-id="${player_id}"]`);
    existingCircles
        .transition()
        .duration(trans_time)
        .style("opacity", 0)
        .remove();
    existingTexts
        .transition()
        .duration(trans_time)
        .style("opacity", 0)
        .remove();


    delete node_players[player_id];
}

source.addEventListener('initialize', function(e) {
    const data = JSON.parse(e.data);
    const sender_id = data.sender_id;
    node_polygons[sender_id] = { polygon: data.polygon, site: data.site };

    Object.keys(node_polygons).forEach(sender_id => {
        updatePolygon(sender_id);
    });

    const players = data.players;
    players.forEach((player) => {
        const player_id = player.player_id;
        const { x, y } = player;
        node_players[player_id] = { x, y, sender_id: sender_id };
        console.log("TRYING TO UPDATE PLAYER");
        updatePlayer(player_id);
    });

    console.log(node_players);
    console.log('Initialization');
}, false);


source.addEventListener('polygon_add', function(e) {
    const data = JSON.parse(e.data);
    node_polygons[data.sender_id] = {polygon: data.polygon, site: data.site};
    console.log('Polygon added');
    updatePolygon(data.sender_id);
}, false);

source.addEventListener('polygon_update', function(e) {
    const data = JSON.parse(e.data);
    const sender_id = data.sender_id;
    if (node_polygons.hasOwnProperty(sender_id)) {
        node_polygons[sender_id] = { polygon: data.polygon, site: data.site };
        console.log('Polygon updated');
        updatePolygon(sender_id);
    }
}, false);

source.addEventListener('node_leave', function(e) {
    const data = JSON.parse(e.data);
    const node = node_polygons[data];
    if(node){
        removePolygon(data)
    }
    console.log('Node left');
}, false);


source.addEventListener('player_add', function(e) {
    const data = JSON.parse(e.data);
    const player=data.player;
    const player_id = player.player_id;
    node_players[player_id] = { x: player.x, y: player.y,sender_id :data.sender_id};
    console.log('Player added');
    updatePlayer(player_id);
}, false);


source.addEventListener('player_update', function(e) {
    const data = JSON.parse(e.data);
    const player=data.player;
    const player_id = player.player_id;
    if (node_players.hasOwnProperty(player_id)) {
        node_players[player_id] = { x: player.x, y: player.y,sender_id :data.sender_id};
        console.log('Player updated');
        updatePlayer(player_id);
    }
}, false);

source.addEventListener('player_leave', function(e) {
    const data = JSON.parse(e.data);
    const player = node_players[data];
    if(player){
        removePlayer(data)
    }
    console.log('Player left');
}, false);

