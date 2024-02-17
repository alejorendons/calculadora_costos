use std::io::{self, Write};

struct RecursoHumano {
    valor_por_hora: f64,
    horas_trabajadas: f64,
    viaticos: f64,
}

struct CostoInfraestructura {
    nube: f64,
    software: f64,
    otros: f64,
}

struct Empresa {
    nombre: String,
    nit: String,
    direccion: String,
    telefono: String,
    descripcion_software: String,
    recursos_humanos: Vec<RecursoHumano>,
    infraestructura: CostoInfraestructura,
    iva: f64,
    retencion: f64,
    reteica: f64,
    costo_total_antes_impuestos: f64,
    costo_total: f64,
}

fn main() {
    let mut empresas: Vec<Empresa> = Vec::new();

    loop {
        println!("¿Qué deseas hacer?");
        println!("1. Agregar nueva empresa");
        println!("2. Ver historial de empresas");
        println!("3. Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer la línea");
        let opcion: i32 = opcion.trim().parse().expect("Por favor, introduce un número válido.");

        match opcion {
            1 => {
                if let Err(e) = agregar_empresa(&mut empresas) {
                    println!("Error al agregar empresa: {}", e);
                }
            },
            2 => ver_historial(&empresas),
            3 => {
                println!("Saliendo del programa.");
                break;
            },
            _ => println!("Opción no válida, por favor intenta de nuevo."),
        }

    }
}




fn agregar_empresa(empresas: &mut Vec<Empresa>) -> Result<(), &'static str> {
    let nombre = leer_entrada("Introduce el nombre de la empresa:")?;
    let nit = leer_entrada("Introduce el NIT de la empresa:")?;
    let direccion = leer_entrada("Introduce la dirección de la empresa:")?;
    let telefono = leer_entrada("Introduce el teléfono de la empresa:")?;
    let descripcion_software = leer_entrada("Describe el software que deseas:")?;

    println!("Introduce el número de personas que trabajarán en el proyecto:");
    let cantidad_personas: u32 = leer_entrada_numerica("Cantidad de personas:")? as u32;

    let mut recursos_humanos = Vec::new();
    for _ in 0..cantidad_personas {
        println!("Introduciendo datos de la persona:");
        let valor_por_hora = leer_entrada_numerica("Introduce el valor por hora:")?;
        let horas_trabajadas = leer_entrada_numerica("Introduce las horas trabajadas:")?;
        let viaticos = if leer_entrada("¿Tiene viáticos? (si/no):")?.to_lowercase() == "si" {
            leer_entrada_numerica("Introduce el valor de los viáticos:")?
        } else {
            0.0
        };

        recursos_humanos.push(RecursoHumano {
            valor_por_hora,
            horas_trabajadas,
            viaticos,
        });
    }

    let nube = leer_entrada_numerica("Introduce el costo de la nube:")?;
    let software = leer_entrada_numerica("Introduce el costo del software:")?;
    let otros = leer_entrada_numerica("Introduce otros costos de infraestructura:")?;

    let infraestructura = CostoInfraestructura { nube, software, otros };

    let costo_recursos_humanos: f64 = recursos_humanos.iter().map(|rh| rh.valor_por_hora * rh.horas_trabajadas + rh.viaticos).sum();
    let costo_infraestructura: f64 = nube + software + otros;
    let costo_total_antes_impuestos = costo_recursos_humanos + costo_infraestructura;

    let iva = costo_total_antes_impuestos * 0.19;
    let retencion = costo_total_antes_impuestos * 0.10;
    let reteica = costo_total_antes_impuestos * 0.01;
    let costo_total = costo_total_antes_impuestos + iva + retencion + reteica;

    empresas.push(Empresa {
        nombre: nombre.clone(),
        nit: nit.clone(),
        direccion: direccion.clone(),
        telefono: telefono.clone(),
        descripcion_software: descripcion_software.clone(),
        recursos_humanos,
        infraestructura,
        iva,
        retencion,
        reteica,
        costo_total_antes_impuestos,
        costo_total,
    });

    println!("\nResumen de Costos para la Empresa: {}", nombre);
    println!("Costo Total Antes de Impuestos: {:.2}", costo_total_antes_impuestos);
    println!("IVA (19%): {:.2}", iva);
    println!("Retención en la fuente (10%): {:.2}", retencion);
    println!("ReteICA (1%): {:.2}", reteica);
    println!("Costo Total (Incluyendo Impuestos): {:.2}", costo_total);

    Ok(())
}

fn leer_entrada(prompt: &str) -> Result<String, &'static str> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer la línea");
    if entrada.trim().is_empty() {
        Err("La entrada no puede estar vacía.")
    } else {
        Ok(entrada.trim().to_string())
    }
}

fn leer_entrada_numerica(prompt: &str) -> Result<f64, &'static str> {
    leer_entrada(prompt)?
        .parse::<f64>()
        .map_err(|_| "Por favor, introduce un número válido.")
}

fn ver_historial(empresas: &[Empresa]) {
    if empresas.is_empty() {
        println!("No hay empresas en el historial.");
        return;
    }

    for (index, empresa) in empresas.iter().enumerate() {
        println!("Empresa {}: Nombre: {}, NIT: {}, Dirección: {}, Teléfono: {}, Descripción del Software: {}, Costo Total Antes de Impuestos: {:.2}, IVA: {:.2}, Retención: {:.2}, ReteICA: {:.2}, Costo Total: {:.2}", 
                 index + 1, 
                 empresa.nombre, 
                 empresa.nit, 
                 empresa.direccion, 
                 empresa.telefono, 
                 empresa.descripcion_software,
                 empresa.costo_total_antes_impuestos,
                 empresa.iva,
                 empresa.retencion,
                 empresa.reteica,
                 empresa.costo_total);
    }
}

