use std::io;

pub fn ask_to_send() -> bool {
    println!("¿Desea enviar los datos de la prueba al servidor? (y/n)");
    println!("(Tu información solo se usará para generar un reporte en Discord y no se almacenará en ninguna base de datos.)");
    let mut respuesta: String = String::new();

    loop {
        match io::stdin().read_line(&mut respuesta) {
            Ok(_) => {
                let input = respuesta.trim().to_lowercase();
                if input == "y" {
                    return true;
                } else if input == "n" {
                    return false;
                } else {
                    println!("Entrada no válida. Por favor, presione 'y' para sí o 'n' para no.");
                    respuesta.clear();
                }
            }
            Err(_) => {
                println!("Error al leer la entrada. Intente nuevamente.");
                respuesta.clear();
            }
        }
    }
}

pub fn display_banner() {
    let banner: &str = r#"
                                                                                      #++++**
     _____ ______ _   _  ______                 _                          _        -*----#
    /  __ \| ___ \ | | | | ___ \               | |                        | |       *----#
    | /  \/| |_/ / | | | | |_/ / ___ _ __   ___| |__  _ __ ___   __ _ _ __| | __   -*---*+=== 
    | |    |  __/| | | | | ___ \/ _ \ '_ \ / __| '_ \| '_ ` _ \ / _` | '__| |/ /   *-------*=
    | \__/\| |   | |_| | | |_/ /  __/ | | | (__| | | | | | | | | (_| | |  |   <    *++++--*:
     \____/\_|    \___/  \____/ \___|_| |_|\___|_| |_|_| |_| |_|\__,_|_|  |_|\_\       *==*
                                                                                     -*++  
                                                                                     ##:
                                                                                    =#"#;
    println!("\x1B[34m{}\x1B[0m", banner);
} 