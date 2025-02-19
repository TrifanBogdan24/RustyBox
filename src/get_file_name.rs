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

fn main() {
    let mut cale: String = String::from("dir1/dir2/dir3/file1");
    let mut file: String = nume_fisier(cale.clone());
    println!("{:?} {:?}", cale, file);

    cale = String::from("/file1");
    file = nume_fisier(cale.clone());
    println!("{:?} {:?}", cale, file);

    cale = String::from("file1");
    file = nume_fisier(cale.clone());
    println!("{:?} {:?}", cale, file);
}