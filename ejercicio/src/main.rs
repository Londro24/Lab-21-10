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

fn mas_menos_ranking(arreglo: [[&str;5];51]) {
    let mut _numero_1 = 0;
    let mut _numero_2 = 0;
    let mut mayor = 0;
    let mut menor = 100;
    let mut buscador_mayor = 0;
    let mut buscador_menor = 0;

    for line in 0..49 {
        _numero_1 = arreglo[line][4].parse().unwrap();
        _numero_2 = arreglo[line + 1][4].parse().unwrap();
        if _numero_1 > _numero_2 && (_numero_1 > mayor || _numero_2 > mayor) {
            mayor = _numero_1;
        } else if _numero_2 > _numero_1 && (_numero_1 > mayor || _numero_2 > mayor) {
            mayor = _numero_2;
            buscador_mayor = line + 2
        }
        
        if _numero_1 < _numero_2 && (_numero_1 < menor || _numero_2 < menor) {
            menor = _numero_1;
        } else if _numero_2 < _numero_1 && (_numero_1 < menor || _numero_2 < menor) {
            menor = _numero_2;
            buscador_menor = line + 2
        }
    }

    println!("la pista mas popular es la n°{}, que es {}, con una popularidad de {} puntos", buscador_mayor, arreglo[buscador_mayor- 1][1], mayor);
    println!("la pista menos popular es la n°{}, que es {}, con una popularidad de {} puntos", buscador_menor, arreglo[buscador_menor- 1][1], menor);
}

fn main() {
    let path = Path::new("top50.csv");
    let texto = open_file_to_read(path);
    let mut arreglo: [[&str;5];51] = [["";5];51];
    //arreglo = hacer_arreglo(texto, arreglo);

    {
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
    }

    println!("----------------------------------------------------------");
    let split_1 = texto.split("\n");
    for line in split_1 {
        println!("{}", line)
    }
    println!("----------------------------------------------------------");
    mas_menos_ranking(arreglo);
    println!("----------------------------------------------------------");

}
