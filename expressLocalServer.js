const { exec } = require("child_process");
let express = require('express');
let app = express();
let opn = require('opn');
let fetch = require('node-fetch');
let faceapi = require('face-api.js');
let canvas = require('canvas');
let path = require('path');
faceapi.env.monkeyPatch({ fetch: fetch });
const { Canvas, Image, ImageData } = canvas
faceapi.env.monkeyPatch({ Canvas, Image, ImageData })

let takePicture = () => {
	let ret;
	exec("python python/cameracapture.py", (error, stdout, stderr) => {
		ret = stdout;
	});
	return ret;
}

function face_api() {

	let input_src = takePicture();
	let img = new Image();
	img.src = input_src;

	const MODELS_URL = './models';
	Promise.all([
		faceapi.nets.ssdMobilenetv1.loadFromDisk(MODELS_URL),
		faceapi.nets.faceRecognitionNet.loadFromDisk(MODELS_URL),
		faceapi.nets.faceLandmark68Net.loadFromDisk(MODELS_URL),
		faceapi.nets.faceExpressionNet.loadFromDisk(MODELS_URL),
	]).then(start).catch(err => console.log(err));

	function start(){
		faceapi.detectSingleFace(img).withFaceExpressions().then(x => console.log(x.expressions));
		console.log('loaded.');
	}
}

app.use(express.static('client'));

face_api()
let server = app.listen(8080, () => {
	let host = server.address().address;
    let port = server.address().port;

	console.log(`Express App listening at http://localhost:${port}`);
})

//opn('http://localhost:8080');

