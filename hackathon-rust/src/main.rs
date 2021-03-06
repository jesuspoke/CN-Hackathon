#![allow(dead_code)]

fn call_node<S : AsRef<std::ffi::OsStr>>(script : S) -> Vec<u8> {
	use std::process::Command;
	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(&["/C".as_ref(), "npm run".as_ref(), script.as_ref()])
			.output()
			.expect("failed to execute process")
			.stdout
	} else {
		Command::new("sh")
			.arg("-c")
			.arg("npm run")
			.arg(script)
			.output()
			.expect("failed to execute process")
			.stdout
	}
}

fn call_node_string<S : AsRef<std::ffi::OsStr>>(script : S) -> String {
	call_node(script).into_iter().map(|i| i as char).collect::<String>()
}

fn call_python<S : AsRef<std::ffi::OsStr>>(script : S) -> Vec<u8> {
	use std::process::Command;
	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(&["/C".as_ref(), "python".as_ref(), script.as_ref()])
			.output()
			.expect("failed to execute process")
			.stdout
	} else {
		Command::new("sh")
			.arg("-c")
			.arg("python")
			.arg(script)
			.output()
			.expect("failed to execute process")
			.stdout
	}
}

fn call_python_string<S : AsRef<std::ffi::OsStr>>(script : S) -> String {
	call_python(script).into_iter().map(|i| i as char).collect::<String>()
}

fn send_notif(title : &str, body : &str) {
	use notify_rust::Notification;
	Notification::new()
	    .summary(title)
	    .body(body)
	    .show().unwrap();
}

fn get_pred() {
	call_node_string("readFace");
}

fn run() {
	let mut map = std::collections::HashMap::<String, f64>::new();
	let input = std::fs::read_to_string("../client/results.json").unwrap();
	let mut json_object = json::parse(input.as_str()).unwrap();
	let results = &mut json_object["results"];

	for _ in 0..std::cmp::max(0, results.len() as isize - 2) {
		results.array_remove(0);
	}

	for _ in 0..std::cmp::max(0, results.len() as isize - 1) {
		let result = results.array_remove(0);
		if result == json::JsonValue::Null {
			break;
		}

		let app = format!("{}", result["app"]);

		let happy = result["data"]["happy"].as_f64().unwrap();
		let sad = result["data"]["sad"].as_f64().unwrap();

		let disgusted = result["data"]["disgusted"].as_f64().unwrap();
		let angry = result["data"]["angry"].as_f64().unwrap();
		let fearful = result["data"]["fearful"].as_f64().unwrap();


		if !map.contains_key(&app) {
			map.insert(app.clone(), 0.0);
		}

		let score_mut = map.get_mut(&app).unwrap();
		*score_mut += happy;
		*score_mut -= sad + angry + fearful + disgusted;

	}

	let result = results.array_remove(0);
	let app = format!("{}", result["app"]);
	if map.contains_key(&app) {
		let score = map.get(&app).unwrap();
		println!("score : {}", *score);
		if *score < -0.0 {
			send_notif("You seem upset.", format!("Try taking a break from using {}, ", app).as_str());
		}
	}
}

fn main() {
	loop {
		call_node("readFace");
		run();
		std::thread::sleep(std::time::Duration::from_secs(10));
	}
}