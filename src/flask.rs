
use crate::helpers::get_current_working_dir;
use crate::helpers::create_dir;
use crate::helpers::create_file_and_write;
use crate::helpers::get_app_content;
use crate::helpers::get_init_content;
use crate::helpers::get_route_content;

use std::path::PathBuf;
trait Create {
    fn create_dirs(&self);
    fn create_requirements(&self);
    fn create_python_files(&self);
}
// 
struct FlaskProject{
    name: String,
    directories: [&'static str; 2],
    current_dir: String,
    project_dir: String
}

impl FlaskProject {
    pub fn new(project_name: String) -> Self { 
        let curr_dir = get_current_working_dir();
        let mut path = PathBuf::new();
        path.push(curr_dir.to_string());
        // clone is a bad pracitse 
        path.push(project_name.clone());
        let project_dir = path.into_os_string().into_string().unwrap();
         Self { 
             name: project_name,
             directories: ["templates", "routes"],
             current_dir: get_current_working_dir(),
             project_dir: project_dir
         }
     } 
} 

impl Create for FlaskProject {
    fn create_dirs(&self) {

        for dirs in self.directories.into_iter(){
            let mut path = PathBuf::new();
            path.push(&self.project_dir);

            path.push(&self.name);

            path.push(dirs);


            create_dir(path.into_os_string().into_string().unwrap());
        }
    }

    fn create_python_files(&self) {
        println!("Populating files");
        let mut init_path = PathBuf::new();
        init_path.push(self.project_dir.to_string());
        
        init_path.push(&self.name);
        init_path.push("__init__.py");
        // println!("Init Path is {}",init_path.display() );
        println!("Init Path is {}",init_path.display() );

        create_file_and_write(&init_path.into_os_string().into_string().unwrap(), get_init_content(self.name.clone()).as_bytes());

        let mut route_path = PathBuf::new();
        route_path.push(self.project_dir.to_string());
        route_path.push(&self.name);
        route_path.push("routes");
        route_path.push("hello.py");
        create_file_and_write(&route_path.into_os_string().into_string().unwrap(), get_route_content(self.name.clone()).as_bytes());


        let mut app_path = PathBuf::new();
        app_path.push(self.project_dir.to_string());
        app_path.push("app.py");
        create_file_and_write(&app_path.into_os_string().into_string().unwrap(), get_app_content(self.name.clone()).as_bytes());

    }

    fn create_requirements(&self) {
        println!("Creating requirements.txt");
        let mut path = PathBuf::new();
        path.push(self.current_dir.to_string());
        path.push(&self.name);
        path.push("requirements.txt");
        let requirements: &str = "Flask";
        create_file_and_write(&path.into_os_string().into_string().unwrap(),requirements.as_bytes())
    }




}

pub fn flask_init(project_name: String){
    println!("Project Name is {}", project_name);
    // get current directory
    let project = FlaskProject::new(project_name);
    project.create_dirs();
    project.create_requirements();
    project.create_python_files();
}