enum IsbnE {
    Isbn10,
    Isbn13,
}

/// Determines whether the supplied string is a valid ISBN10 0r ISBN13 number

pub fn is_valid_isbn(isbn: &str) -> bool {

    let isbn_s =isbn.to_string().replace("-", "");
    let lenght_isbn =isbn_s.chars().count();

    let isbn_type= if lenght_isbn==10 { IsbnE::Isbn10 }
    else if lenght_isbn==13 { IsbnE::Isbn13 }
    else{return false };

    let res = match isbn_type{
    IsbnE::Isbn10 => is_valid_isbn10(isbn_s) ,
    IsbnE::Isbn13 => is_valid_isbn13(isbn_s)
    };
    return res;
}

pub fn is_valid_isbn10(isbn: String)->bool{


    let mut tot=0u32;
    let mut i=1u32;

    for c in isbn.to_ascii_uppercase().chars().rev(){
        let sum = match c{
            '0'..='9' => {c.to_digit(10).unwrap()*i},
            'X' if i==1 => {10*i}
            _ => {return false},
        };

        tot+=sum;
        i+=1;
    }
    if tot%11==0 { return true }
    else {return false }

}


pub fn is_valid_isbn13(isbn: String)->bool{



    let mut tot=0u32;
    let mut i=1u32;

    for c in isbn.to_ascii_uppercase().chars().rev(){
        let sum = match c{
            '0'..='9' if i%2==0 => {c.to_digit(10).unwrap()*3},
            '0'..='9' if i%2!=0 => {c.to_digit(10).unwrap()*1},
            'X' if i==1 => {10*i}
            _ => {return false},
        };

        tot+=sum;
        i+=1;

    }
    if tot%10==0 { return true }
    else {return false }

}