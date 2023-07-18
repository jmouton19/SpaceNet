// Establish a connection to the server

const source = new EventSource('http://127.0.0.1:3030/api');

const trans_time=500;

let node_polygons={};

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
    updatePolygons();
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
            existingPolygons.attr("data-sender-id", sender_id);

            existingCircles
                .transition()
                .duration(trans_time) // Set the duration of the transition in milliseconds
                .attr("cx", site[0])
                .attr("cy", site[1]);

            // Update the data attribute of the existing circles
            existingCircles.attr("data-sender-id", sender_id);
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


// Define event handlers
source.addEventListener('initialize', function(e) {
    const data = JSON.parse(e.data);
    node_polygons[data.sender_id] = {polygon: data.polygon, site: data.site};

    console.log('Initialization');
    Object.keys(node_polygons).forEach(sender_id => {
        updatePolygon(sender_id);
    });
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
    // Handle player_add event
    console.log('Player added:', data);
}, false);

source.addEventListener('player_update', function(e) {
    const data = JSON.parse(e.data);
    // Handle player_update event
    console.log('Player updated:', data);
}, false);

source.addEventListener('remove_player', function(e) {
    const data = JSON.parse(e.data);
    // Handle remove_player event
    console.log('Player removed:', data);
}, false);

