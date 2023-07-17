// Establish a connection to the server
const source = new EventSource('http://127.0.0.1:3030/api');


let node_polygons={};

// Define event handlers
source.addEventListener('initialize', function(e) {
    const data = JSON.parse(e.data);
    node_polygons[data.sender_id] = {polygon: data.polygon, site: data.site};

    console.log('Initialization:',data);
    console.log('NEW MAP:',node_polygons);
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

source.addEventListener('polygon_update', function(e) {
    const data = JSON.parse(e.data);
    node_polygons[data.sender_id] = {polygon: data.polygon, site: data.site};

    console.log('Polygon updated:',data);
    console.log('NEW MAP:',node_polygons);
}, false);

source.addEventListener('node_leave', function(e) {
    const data = JSON.parse(e.data);
    delete node_polygons[data];

    console.log('Node left:',data);
    console.log('NEW MAP:',node_polygons);
}, false);