/*
    Seção 08 - Iteradores

*/


/*
    s08_a01 - Introdução aos Iteradores

Iteradores

	Definidos no módulo std::iter
	https://doc.rust-lang.org/std/iter/

	trait Iterator {
    	type Item;
	    fn next(&mut self) -> Option<Self::Item>;
	}
	
	Enquanto existirem elementos para serem retornados 'next()' retorna o próximo elemento da iteração.
	O elemento aparece na forma 'Some(Item)'.
	Quando não existirem mais elementos	na iteração é retornado 'None'.

	Existem 3 métodos comuns para criar iteradores a partir de uma coleção:

	iter()
    	Itera sobre um empréstimo (referência) dos elementos

	iter_mut()
    	Itera sobre um empréstimo (referência) mutável dos elementos

	into_iter()
		Itera sobre a propriedade dos elementos

	Método muito usado:

	for_each()
		Aplica uma closure a cada elemento do iterador
*/



// s08_a02 - Método iter()
fn s08_a02() {
	println!("\ns08_a02 - Método iter()");
	
	let lista = vec![ String::from("aaaa"),
								String::from("bbbb"),
								String::from("cccc"),
								String::from("dddd"),
								String::from("eeee") ];

	println!("!!!lista antes!!! {:?}",lista);

	let itera_lista = lista.iter();

	for val in itera_lista {
		println!("no for: {}", val);
	}

	println!("!!!lista depois!!! {:?}",lista);
}


// s08_a03 - Método for_each()
fn s08_a03() {
	println!("\ns08_a03 - Método for_each()");
	
	let lista = vec![ String::from("aaaa"),
						String::from("bbbb"),
						String::from("cccc"),
						String::from("dddd"),
						String::from("eeee") ];

	println!("!!!lista antes!!! {:?}",lista);

    for val in lista.iter() {
		println!("no for: {}", val);
	}

	lista.iter().for_each(|x| println!("na closure: {}", x) );

	println!("!!!lista depois!!! {:?}",lista);
}


// s08_a04 - Método iter_mut()
fn s08_a04() {
	println!("\ns08_a04 - Método iter_mut()");
	
	let mut lista = vec![ String::from("aaaa"),
						String::from("bbbb"),
						String::from("cccc"),
						String::from("dddd"),
						String::from("eeee") ];

	println!("!!!lista antes!!! {:?}",lista);
	
	for val in lista.iter_mut() {
		val.push_str("@@");
	}

	lista.iter_mut().for_each(|x| { println!("--> {}", x);
												 x.push_str("##"); } );

	println!("!!!lista depois!!! {:?}",lista);
}



// s08_a05 - Método into_iter()
fn s08_a05() {
	println!("\ns08_a05 - Método into_iter()");

	let lista = vec![ String::from("aaaa"),
						String::from("bbbb"),
						String::from("cccc"),
						String::from("dddd"),
						String::from("eeee") ];

	println!("!!!lista antes!!! {:?}",lista);

	lista.into_iter().for_each(|mut x| { println!("--> {}", x);
												 x.push_str("####");
												 println!("--> {}", x);
												} );

	//for mut val in lista.into_iter() {
	//	val.push_str("***");
	//	println!("no for: {}", val);
	//}

	//println!("!!!lista depois!!! {:?}",lista);     // borrow of moved value: `lista`
}


// s08_a06 - Exemplo do Livro no Listing 13-13
fn s08_a06() {
	println!("\ns08_a06 - Exemplo do Livro no Listing 13-13");

	let v1 = vec![1, 2, 3];

	let v1_iter = v1.iter();

	let total: i32 = v1_iter.sum();

	println!("total: {}", total);

	println!("!!!v1!!! {:?}", v1);

	// sum() soma cada item a um totalizador, o qual é retornado ao final
	// Está implementado para vários tipos numéricos

	let lista = vec![ String::from("aaaa"),
						String::from("bbbb"),
						String::from("cccc"),
						String::from("dddd"),
						String::from("eeee") ];

	//let total2: String = lista.iter().sum();		// 'sum' não definido para &String
	//println!("total2: {}", total2);

}



fn main() {
    println!("\nSeção 08 - Iteradores");
	s08_a02();
	s08_a03();
	s08_a04();
	s08_a05();
	s08_a06();
}




