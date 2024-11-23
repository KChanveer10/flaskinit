use std::env;
use std::fmt::format;
use std::fs;
use std::path::PathBuf;


pub fn get_current_working_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}

pub fn collect_user_arguments() -> Vec<String> {
    env::args().collect()
}

fn help(){
    println!("To use this call it");
    println!("flaskinit <project_name>");
}

pub fn check_if_arguments_count_valid(args: &Vec<String>) -> bool {
    if args.len() == 2 {
        return true
    }
    help();
    return false
}

pub fn create_dir(path:String) {
    // prject_name/templates
    fs::create_dir_all(path).unwrap()
}   

pub fn create_file_and_write(file_path: &str, content: &[u8]){
    fs::File::create(file_path).unwrap();
    fs::write(file_path, content).unwrap(); 
}

pub fn get_route_content(project_name: String) -> String{
    let route_content = format!(
        concat!(
            "from flask import Flask, render_template, request, session\n",
            "from {} import app\n",
            "@app.get('/test')\n",
            "def test():\n",
            " return \" hello\""
        ),project_name);
    return route_content
}

pub fn get_init_content(project_name: String) -> String{
    let init_content = format!(
        concat!(
            "from flask import Flask\n",
            "app = Flask(__name__)\n",
            "from {}.routes import hello"
        )
    , project_name);
    return init_content;
}

pub fn get_app_content(project_name: String) -> String{
    let app_content = format!(
        concat!(
            "from {} import app\n",
            "if __name__ == '__main__':\n",
            "   app.run(debug=True)"
        )
        , project_name
    );
    return app_content
}
// this is incorrec implenetation
pub fn convert_pb_to_str(path: PathBuf) -> String{
    return path.into_os_string().into_string().unwrap();
}