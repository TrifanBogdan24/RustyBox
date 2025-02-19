use std::env;                                       // pentru argumentele din linia de comanda,pwd
use std::fs;                                        // pentru cat, mkdir, ln (hard link), chmod
use std::path::Path;                                // pentru a verifica existenta unui fisier / director, chmod
use std::os::unix::fs::symlink as symbolic_link;    // pentru ln -s
use std::os::unix::fs::PermissionsExt;              // pentru chmod

use regex::Regex;                                   // pentru grep

// use chrono::Local;                               // pentru touch

fn pwd() {

    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir_path) = current_dir.to_str() {
            println!("{}", dir_path);
        }
    }

    return ();
}

fn echo(args: Vec<String>) -> Result<i32, i32> {

    if args.len() == 2 {
        // ./rustybox echo
        println!();
        return Ok(0);
    }

    if args.len() == 3 && args[2] == "-n" {
        // ./rustybox echo -n
        print!("");
        return Ok(0);
    }

    let start;
    if args[2] == "-n" {
        start = 3;
    } else {
        start = 2;
    }

    for i in start..=(args.len() - 1) {
        print!("{}", &args[i]);
        if i != args.len() - 1 {
            print!(" ");
        }
    }

    if args[2] != "-n" {
        println!("");
    }

    return Ok(0);
}

fn cat(args: Vec<String>) -> Result<i32, i32> {
    
    if args.len() == 2 {
        // parametri insuficienti (./rustybox cat)
        return Err(-1);     // comanda invalida
    }

    for i in 2..=(args.len() - 1) {

        // dam cat pe fiecare argument in parte
        match fs::read_to_string(&args[i]) {
            Ok(continut) => {
                print!("{}", continut);
            }
            Err(_) => {
                return Err(1);      // comanda nu s - a executat cu succes
            }
        }
    }

    return Ok(0);
}

fn mkdir(args: Vec<String>) -> Result<i32, i32> {
    
    if args.len() == 2 {
        // parametri insuficienti (./rustybox mkdir)
        return Err(-1);
    }

    for i in 2..=(args.len() - 1) {
        if let Err(_) = fs::create_dir_all(&args[i]) {
            return Err(1);
        }

    }

    return Ok(0);
}

fn mv(args: Vec<String>) -> Result<i32, i32> {

    if args.len() <= 3 {
        // parametri insufcicienti
        return Err(-1);
    }

    for i in 2..=(args.len() - 2) {
        if let Err(_) = fs::rename(&args[i], &args[args.len() - 1]) {
            return Err(1);
        } 
    }


    return Ok(0);
}

fn ln(args: Vec<String>) -> Result<i32, i32> {

    if args.len() < 4 || args.len() > 5 {
        return Err(-1);
    }

    // un link symbolic va primi exact 5 argument : al trilea va fi -s
    
    if args.len() == 5 && args[2] != "-s" && args[2] != "--symbolic" {
        // comanda invalida : ./rustybox ln ceva file1 file2
        return Err(-1);
    }


    if args[2] == "-s" || args[2] == "--symbolic" {
        // synmolic link : ./rustybox ln -s file1 file_link
        if let Err(_) = symbolic_link(&args[3], &args[4]) {
            return Err(1);
        }


    } else {
        // hard link : ./rustybox ln file1 file_link
        if let Err(_) = fs::hard_link(&args[2], &args[3]) {
            return Err(1);
        }
    }

    return Ok(0);
}

fn rmdir(args: Vec<String>) -> Result<i32, i32> {

    if args.len() <= 2 {
        return Err(-1);
    }

    for i in 2..=(args.len() - 1) {
        if let Err(_) = fs::remove_dir(&args[i]) {
            return Err(1);
        }
    }

    return Ok(0);
}



fn rm(args: Vec<String>) -> Result<i32, i32> {
    // pentru rm ordinea argumentelor nu contezeaza
    // rm nu opreste executia daca intampina o eroare

    if args.len() < 3 {
        return Err(-1);
    }

    let mut rm_recursiv: bool = false;
    let mut rm_director_gol: bool = false;
    
    let mut flag_recursiv: Vec<String> = Vec::new();
    flag_recursiv.push("-r".to_string());
    flag_recursiv.push("-R".to_string());
    flag_recursiv.push("--recursive".to_string());
    // println!("flagurile posibile pentru stegerea recursiva a directoarelor : {:?}", flag_recursiv);

    let mut flag_director_gol: Vec<String> = Vec::new();
    flag_director_gol.push("-d".to_string());
    flag_director_gol.push("-D".to_string());
    flag_director_gol.push("--dir".to_string());

    // println!("flagurile posibile pentru stegerea directoarelor goale : {:?}", flag_director);


    // verificam existenta flagurilor si ne dam seama tipul de rm
    let mut start_pos = 2;
    
    if args.len() >= 3 && flag_recursiv.contains(&args[2]) == true {
        rm_recursiv = true;
        start_pos = 3;

        if args.len() == 3 {
            // rustybox rm -r
            return Err(-1);    // invalid command
        }
    }

    if args.len() >= 3 && flag_director_gol.contains(&args[2]) == true {
        rm_director_gol = true;
        start_pos = 3;

        if args.len() == 3 {
            // rustybox rm -d
            return Err(-1);    // invalid command
        }
    }

    if args.len() >= 4 && flag_recursiv.contains(&args[3]) == true {
        rm_recursiv = true;
        start_pos = 4;

        if flag_recursiv.contains(&args[2]) == false && flag_director_gol.contains(&args[2]) == false {
            // rustybox rm dir1 -r
            return Err(-1);     // invalid command
        }

        if args.len() == 4 {
            // rustybox rm -d -r
            return Err(-1);    // invalid command
        }
    }

    if args.len() >= 4 && flag_director_gol.contains(&args[3]) == true{
        rm_director_gol = true;
        start_pos = 4;

        if flag_recursiv.contains(&args[2]) == false && flag_director_gol.contains(&args[2]) == false {
            // rustybox rm dir1 -d
            return Err(-1);     // invalid command
        }

        if args.len() == 4 {
            // rustybox rm -d -r
            return Err(-1);    // invalid command
        }
    }
    

    // iteram celelalalte argumente si incercam sa stergem toate fisierele / directoarele

    let mut avem_eroare: bool = false;

    for i in start_pos..=(args.len() - 1) {

        if Path::new(&args[i]).is_dir() == true {
            // stergem un director

            if rm_recursiv == true {
                // rm -r dir1
                // fs::remove_dir_all sterge recursiv un director
                if let Err(_) = fs::remove_dir_all(&args[i]) {
                    avem_eroare = true;
                }
            } else {
                // recursiv == false
                if rm_director_gol == true {
                    // rm --dir dir1 <=> rmdir dir1
                    if let Err(_) = fs::remove_dir(&args[i]) {
                        avem_eroare = true;
                    }
                } else {
                    // recurisv == false && director_gol == false
                    avem_eroare = true;
                }
            }
        } else {
            // stergem un fisier
            if let Err(_) = fs::remove_file(&args[i]) {
                avem_eroare = true;
            }
        }
    }

    if avem_eroare == true {
        return Err(1);      // eroare de executie
    }

    return Ok(0);
}


fn copy_recursiv(src: &str, dest: &str) -> Result<(), ()> {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    if src_path.is_dir() {
        if !dest_path.exists() {
            match fs::create_dir(dest_path) {
                Ok(()) => (),
                Err(_) => return Err(()),
            }
        }

        for entry in fs::read_dir(src_path).map_err(|_| ())? {
            let entry = entry.map_err(|_| ())?;
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().ok_or(())?;
            let dest_entry_path = dest_path.join(entry_name);

            if entry_path.is_dir() {
                match copy_recursiv(&entry_path.to_string_lossy(), &dest_entry_path.to_string_lossy()) {
                    Ok(()) => (),
                    Err(_) => return Err(()),
                }
            } else {
                if let Err(_) = fs::copy(&entry_path, &dest_entry_path) {
                    return Err(());
                }
            }
        }
    } else {
        if let Err(_) = fs::copy(src_path, dest_path) {
            return Err(());
        }
    }

    return Ok(());
}

fn nume_fisier(cale: String) -> String {
    /* daca la input avem
    dir1/dir2/file1 -> file1
    file1 -> file1
    */

    let caractere: Vec<char> = cale.chars().collect();
    
    let mut pos = caractere.len() - 1;
    
    while pos > 0 && caractere[pos - 1] != '/' {
        pos = pos - 1;
    }
    
    let mut res: String = String::new();
    for i in pos..=(cale.len() - 1) {
        res.push(caractere[i]);
    }

    return res;
}


fn cp(args: Vec<String>) -> Result<i32, i32> {
    // NU TRECE TOATE TESTELE : cp -r
    if args.len() < 4 || args.len() > 5 {
        return Err(-1);
    }

    
    if args.len() == 4 {
        // ./rustybox cp file1 dir1
       
        if Path::new(&args[3]).exists() == false {
            // cp dir1/file1 file2

            // println!("copiez fisier {:?} ->  cu numele {:?}", args[2], args[3]);
            
            if let Err(_) = fs::copy(&args[2], &args[3]) {
                return Err(1);
            }
        }

        if Path::new(&args[3]).is_dir() == true {
            // cp dir1/file1 dir2/file2
            
            // folosim nume_fisier(args[2].clone());  ca sa nu pierdem owner ship ul pt args[2]
            let fisier_sursa: String = nume_fisier(args[2].clone());

            let mut cale_destinatie: String = String::from("");
            cale_destinatie.push_str(&args[3]);
            
            if args[3].ends_with('/') == false {
                cale_destinatie.push('/');
            }
            
            cale_destinatie.push_str(&fisier_sursa);
    
            // println!("copiez fisier {:?} ->  in directorul {:?}, cu numele {:?}",
            //             args[2], args[3], args[2]);
            
            if let Err(_) = fs::copy(&args[2], &cale_destinatie) {
                return Err(1);
            }
        }
    }

    if args.len() == 5 {
        // ./rustybox cp -r file1/dir1 dir2
    
        if args[2] != "-r" && args[2] != "-R" && args[2] != "--recursive" {
            return Err(-1);
        }

        if Path::new(&args[3]).is_dir() == true {
            // copiaza recursiv un director : cp -r dir1/dir3 dir2/

            let mut cale_destinatie: String = String::new();
            cale_destinatie.push_str(&args[4]);

            if args[4].ends_with('/') == false {
                cale_destinatie.push('/');
            }
            
            if let Err(()) = copy_recursiv(&args[3], &args[4]) {
                return Err(1);
            }
        } else {
            // copiaza un fisier
            
            if Path::new(&args[4]).exists() == false {
                // cp -r dir1/file1 file2
                
                // println!("copiez fisier {:?} ->  cu numele {:?}", args[2], args[3]);
                
                if let Err(_) = fs::copy(&args[3], &args[4]) {
                    return Err(1);
                }
            }
    
            if Path::new(&args[4]).is_dir() == true {
                // cp -r dir1/file1 dir2/
                
                // folosim nume_fisier(args[2].clone());  ca sa nu pierdem owner ship ul pt args[2]
                let fisier_sursa: String = nume_fisier(args[3].clone());

                let mut cale_destinatie: String = String::from("");
                cale_destinatie.push_str(&args[4]);
                
                if args[4].ends_with('/') == false {
                    cale_destinatie.push('/');
                }
                
                cale_destinatie.push_str(&fisier_sursa);
                // println!("copiez fisier {:?} ->  in directorul {:?}, cu numele {:?}",
                //             args[3], args[4], args[3]);
                
                if let Err(_) = fs::copy(&args[3], &cale_destinatie) {
                    return Err(1);
                }
            }
        } 
    }

    return Ok(0);
}


fn chmod(args: Vec<String>) -> Result<i32, i32> {
    // TODO

    if args.len() < 4 {
        return Err(-1);
    }

    // casting de la string la charuri
    let caractere_permisiuni: Vec<char> = args[2].chars().collect();
    let mut caz_numeric: bool = true;
    
    for i in 0..=(caractere_permisiuni.len() - 1) {
        if caractere_permisiuni[i] < '0' || caractere_permisiuni[i] > '7' {
            caz_numeric = false;
        }
    }
    

    if caz_numeric == true {
        // ./rustybox chmod 451 file

        if caractere_permisiuni.len() > 3 {
            return Err(-1);
        }
        
        let mut permisiuni_numerice: u32 = 0;

        for i in 0..=(caractere_permisiuni.len() - 1) {

            match caractere_permisiuni[i] {
                '0' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 0;
                }
                '1' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 1;
                }
                '2' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 2;
                }
                '3' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 3;
                }
                '4' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 4;
                }
                '5' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 5;
                }
                '6' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 6;
                }
                '7' => {
                    permisiuni_numerice = 8 * permisiuni_numerice + 7;
                }
                _ => {
                    return Err(-1);     // permisiune
                }

            }

        }

        // numarul zecimal nu are aceeasi insemnatate cu cel octal
        
        // setez permisiunile pentru fiecare locatie in parte
        for i in 3..=(args.len() - 1) {
            let perm = fs::Permissions::from_mode(permisiuni_numerice);
            if let Err(_) = fs::set_permissions(&args[i], perm) {
                return Err(1);
            }
        }
        
        return Ok(0);
    }
    
    
    
    // permsiuni literare : chmod a+x file1 sau chmod +r
    
    
    // verificam daca al treilea argument reprezeinta operatii valide de permisiuni
    
    if Path::new(&args[2]).exists() == true {
        return Err(-1);     // invalid command
    }

    let mut ugo: Vec<char> = Vec::new();
    let mut rwx: Vec<char> = Vec::new();

    let mut adaugare_permisiuni: bool = false;
    let mut _eliminare_permisiuni: bool = false;
    let mut semn_intalnit: bool = false;
    
    
    let operatii_permisiuni: Vec<char> = args[2].chars().collect();

    for i in 0..=(operatii_permisiuni.len() - 1) {
        match operatii_permisiuni[i] {
            // ugo : (trebuie sa fie inainte de semn)
            'u' => {
                if !ugo.contains(&'u') {
                    ugo.push('u');
                }
                if semn_intalnit == true {
                    return Err(-1);
                }
            }
            'g' => {
                if !ugo.contains(&'g') {
                    ugo.push('g');
                }
                if semn_intalnit == true {
                    return Err(-1);
                }
            }
            'o' => {
                if !ugo.contains(&'o') {
                    ugo.push('o');
                }
                if semn_intalnit == true {
                    return Err(-1);
                }
            }
            'a' => {
                if !ugo.contains(&'u') {
                    ugo.push('u');
                }
                if !ugo.contains(&'g') {
                    ugo.push('g');
                }
                if !ugo.contains(&'o') {
                    ugo.push('o');
                }
                if semn_intalnit == true {
                    return Err(-1);
                }
            }
            // rwx : (trebuie sa fie dupa semn)
            'r' => {
                rwx.push('r');
                if semn_intalnit == false {
                    return Err(-1);
                }
            }
            'w' => {
                rwx.push('w');
                if semn_intalnit == false {
                    return Err(-1);
                }
            }
            'x' => {
                rwx.push('x');
                if semn_intalnit == false {
                    return Err(-1);
                }
            }

            // semnul : trebuie sa fie unic si sa fie ori plus, ori minus
            '+' => {
                adaugare_permisiuni = true;
                
                if semn_intalnit == true {
                    return Err(-1);
                }
                
                semn_intalnit = true;
            }
            '-' => {
                _eliminare_permisiuni = true;
                
                if semn_intalnit  == true {
                    return Err(-1);
                }

                semn_intalnit = true;
                
            }
            _ => {
                // caracter invalid pentru permisiuni
                return Err(-1);
            }
        }
    }

    if ugo.len() == 0 {
        // chmod +x file <=> chmod a+x file <=> chmod ugo+x file
        ugo.push('u');
        ugo.push('g');
        ugo.push('o');
    }

    // println!("{:?} {:?}", ugo, rwx);

    
    for i in 3..=(args.len() - 1) {
        
        match fs::metadata(&args[i]) {
            
            Ok(metadata) => {
                let mut permisiuni_literare: Vec<char> = Vec::new();
                
                // accesam permisiunile initial ale fieacarui fisier / director in parte
                // read for user
                if metadata.permissions().mode() & 0o400 != 0 {
                    permisiuni_literare.push('r');
                } else {
                    permisiuni_literare.push('-');
                }

                // write for user
                if metadata.permissions().mode() & 0o200 != 0 {
                    permisiuni_literare.push('w');
                } else {
                    permisiuni_literare.push('-');
                }

                // execute for uso
                if metadata.permissions().mode() & 0o100 != 0 {
                    permisiuni_literare.push('x');
                } else {
                    permisiuni_literare.push('-');
                }

                // read for group
                if metadata.permissions().mode() & 0o040 != 0 {
                    permisiuni_literare.push('r');
                } else {
                    permisiuni_literare.push('-');
                }

                // write for group
                if metadata.permissions().mode() & 0o020 != 0 {
                    permisiuni_literare.push('w');
                } else {
                    permisiuni_literare.push('-');
                }

                // execute for group
                if metadata.permissions().mode() & 0o010 != 0 {
                    permisiuni_literare.push('x');
                } else {
                    permisiuni_literare.push('-');
                }

                // read for others
                if metadata.permissions().mode() & 0o004 != 0 {
                    permisiuni_literare.push('r');
                } else {
                    permisiuni_literare.push('-');
                }

                // write for others
                if metadata.permissions().mode() & 0o002 != 0 {
                    permisiuni_literare.push('w');
                } else {
                    permisiuni_literare.push('-');
                }

                // execute for others
                if metadata.permissions().mode() & 0o001 != 0 {
                    permisiuni_literare.push('x');
                } else {
                    permisiuni_literare.push('-');
                }



                // modifcam permisiunile prin adaugare / eleminare in functie de al treilea argument

                if ugo.contains(&'u') && rwx.contains(&'r') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[0] = 'r';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[0] = '-';
                    }
                }
                if ugo.contains(&'u') && rwx.contains(&'w') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[1] = 'w';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[1] = '-';
                    }
                }
                if ugo.contains(&'u') && rwx.contains(&'x') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[2] = 'x';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[2] = '-';
                    }
                }
                if ugo.contains(&'g') && rwx.contains(&'r') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[3] = 'r';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[3] = '-';
                    }
                }
                if ugo.contains(&'g') && rwx.contains(&'w') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[4] = 'w';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[4] = '-';
                    }
                }
                if ugo.contains(&'g') && rwx.contains(&'x') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[5] = 'x';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[5] = '-';
                    }
                }
                if ugo.contains(&'o') && rwx.contains(&'r') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[6] = 'r';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[6] = '-';
                    }
                }
                if ugo.contains(&'o') && rwx.contains(&'w') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[7] = 'w';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[7] = '-';
                    }
                }
                if ugo.contains(&'o') && rwx.contains(&'x') {
                    if adaugare_permisiuni == true {
                        permisiuni_literare[8] = 'x';
                    } else {
                        // _eliminare_permisiuni == true
                        permisiuni_literare[8] = '-';
                    }
                }
                

                // print!("{}    {:?}", args[i], permisiuni_literare);
                // println!("-> {:?}", permisiuni_literare);

                // convertim caracterele permisiunilor modificate intr-in numar din baza 8

                let mut permisiuni_numerice = 0o000;
                
                if permisiuni_literare[0] == 'r' {
                    permisiuni_numerice = permisiuni_numerice | 0o400;
                }
                if permisiuni_literare[1] == 'w' {
                    permisiuni_numerice = permisiuni_numerice | 0o200;
                }
                if permisiuni_literare[2] == 'x' {
                    permisiuni_numerice = permisiuni_numerice | 0o100;
                }
                if permisiuni_literare[3] == 'r' {
                    permisiuni_numerice = permisiuni_numerice | 0o040;
                }
                if permisiuni_literare[4] == 'w' {
                    permisiuni_numerice = permisiuni_numerice | 0o020;
                }
                if permisiuni_literare[5] == 'x' {
                    permisiuni_numerice = permisiuni_numerice | 0o010;
                }
                if permisiuni_literare[6] == 'r' {
                    permisiuni_numerice = permisiuni_numerice | 0o004;
                }
                if permisiuni_literare[7] == 'w' {
                    permisiuni_numerice = permisiuni_numerice | 0o002;
                }
                if permisiuni_literare[8] == 'x' {
                    permisiuni_numerice = permisiuni_numerice | 0o001;
                }

                // print!("{:?}  {:?}", ugo, rwx);
                // println!("{}", permisiuni_numerice);

                let perm = std::fs::Permissions::from_mode(permisiuni_numerice);
                if let Err(_) = fs::set_permissions(&args[i], perm) {
                    return Err(1);
                }
            }

            Err(_) => {
                return Err(1);
            }
        }
    }

    return Ok(0);
}

fn touch(args: Vec<String>) -> Result<i32, i32> {
    // TODO

    if args.len() < 2 || args.len() > 6 {
        return Err(-1);     // comanda invalida
    }

    let mut flags: Vec<String> = Vec::new();
    flags.push("-a".to_string());
    flags.push("-m".to_string());
    flags.push("-c".to_string());
    flags.push("--no-create".to_string());

    if flags.contains(&args[args.len() - 1]) == true {
        // ultimul parametru este flag : ./rustybox touch ... -r
        return Err(-1);         // comanda invalida
    }

    let mut update_access: bool = false;         // $ stat -c "%x" file
    let mut update_modify: bool = false;        // $ stat -c "%y" file
    let mut update_change: bool = false;        // $ stat -c "%z" file 

    for i in 2..=(args.len() - 2) {
        match args[i].as_str() {
            "-a" => update_access = true,
            "-m" => update_modify = true,
            "-c" => update_change = true,
            "--no-create" => update_change = true,
            _ => return Err(-1)        // comanda invalida (flag invalid) 
        }
    }

    if Path::new(&args[args.len() - 1]).exists() == false {
        // cream un fisier nou (nici nu conteaza flagurile)
        if let Err(_) = fs::File::create(&args[args.len() - 1]) {
            return Err(1);              // comanda nu a fost executata cu succes
        }

        return Ok(0);
    }
    

    if Path::new(&args[args.len() - 1]).is_file() == false {
        return Err(1);              // comanda nu a fost executata cu succes
    }



    if update_access == true {
        // actualizeaza data de acces a fisierului
        todo!();        
    }

    if update_modify == true {
        // actualizeaza data ultimei modificari a fisierului
        todo!();
    }

    if update_change == true {
        // actualieaza data ultimei schimbari
        todo!();
    }

    return Ok(0);

}


fn grep(args: Vec<String>) {
    // TO DO

    if args.len() < 4 && args.len() > 5 {
        return ();      // comanda este invalida
    }

    let mut contine_expresia: bool = true;
    
    if args.len() == 5 {
        // rustybox grep -i [exp] file
        if args[2] == "-i" {
            // comanda invaluda : rustybox grep cv fisier
        }
        contine_expresia = false;
    }

    if Path::new(&args[args.len() - 1]).is_file() == false {
        return ();
    }

    match fs::read_to_string(&args[args.len() - 1]) {
        Ok(continut) => {
            // putem citi din fisier

            // rustybox grep '[0-9]' fisier
            
            let rgx;        // expresie regex
            
            match Regex::new(&args[args.len() - 2]) {
                Ok(exp) => rgx = exp,
                Err(_) => return (),
            }
            
            for linie in continut.lines() {
                if rgx.is_match(linie) == contine_expresia {
                    println!("{}", linie);
                }
            }
        }
        Err(_) => {
            return ();      // comanda nu s - a executat cu succes
        }
    }

    return ();
}

fn ll() {
    // TODO
    todo!();
}

fn ls(_args: Vec<String>) -> Result<i32, i32> {
    // TODO
    ll();
    todo!();
}

fn main() {
    // TODO: Read the command line arguments
    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        // se foloseste doar executabilul (nu si parametrii)
        println!("Invalid command");
        std::process::exit(-1);
    } else if args.len() >= 2 && args[1] == "pwd" {
        pwd();

    } else if args.len() >= 2 && args[1] == "echo" {
        match echo(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(246),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
       }
    
    } else if args.len() >= 2 && args[1] == "cat" {
        match cat(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(236),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    
    } else if args[1] == "mkdir" {
        // TO DEBUG
        match mkdir(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(226),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "mv" {
        match mv(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(216),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "ln" {
        match ln(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(206),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "rmdir" {
        match rmdir(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(196),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "rm" {
        match rm(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(186),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "ls" {
        // TODO
        match ls(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(176),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "cp" {
        // TODO
        match cp(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(166),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "touch" {
        // TODO
        match touch(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(156),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "chmod" {
        match chmod(args) {
            Ok(_) => (),
            Err(1) => std::process::exit(231),
            Err(-1) => {
                println!("Invalid command");
                std::process::exit(255);
            },
            _ => ()
        }
    } else if args.len() >= 2 && args[1] == "grep" {
        grep(args);
        return ();
    } else {
        // o alta comanda se foloseste
        println!("Invalid command");
        std::process::exit(-1);
    }

}
