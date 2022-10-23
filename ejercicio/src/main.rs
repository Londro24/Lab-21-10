use std::fs::File;
use std::path::Path;
use std::io::Read;

fn read_file(mut f: &File) -> String{
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    return text;
}

fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}

fn open_file_to_read(p: &Path) -> String{
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        let texto = read_file(&file);
        return texto;
    } else {
        create_blank_file(p);
        let texto = "".to_string();
        return texto;
    }

}


fn main() {
    let path = Path::new("top50.csv");
    let texto = open_file_to_read(path);
    let mut arreglo: [[&str;5];51] = [["";5];51];
    //arreglo = hacer_arreglo(texto, arreglo);

    let split_1 = texto.split("\n");
    let mut contador_line = 0;

    for line in split_1 {
        let mut _contador_thing = 0; 
        let split_2 = line.split(",");
        for thing in split_2 {

            if contador_line > 0 {
                arreglo[contador_line-1][_contador_thing] = thing
            }

            _contador_thing += 1;
        }

        contador_line += 1;
        if contador_line == 52 {
            break;
        }
    }

    println!("----------------------------------------------------------");
    for index in 0..52 {
        println!("{:?}", arreglo[index])
    }

}
