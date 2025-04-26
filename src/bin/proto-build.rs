/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::Cmd;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos_path = Path::new("proto");
    let files = Cmd::get_files(protos_path).unwrap();

    for file_path in files.iter() {
        println!("{:?}", file_path);

        tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .out_dir("tmp")
            .compile(&[&file_path], &["proto/"])?;
        //
        let file_name = Cmd::name(file_path).unwrap();
        let module_path = format!("src/tinkoff/api/{file_name}.rs");
        Cmd::replace(
            Path::new("tmp/tinkoff.public.invest.api.contract.v1.rs"),
            Path::new(&module_path),
        )?;
    }

    Ok(())
}
