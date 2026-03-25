use rand::prelude::*;

fn main() {
    let mut vetor: Vec<u32> = gerar_vetor(100000, 10000);
    radix_sort(&mut vetor);
    // Como o vetor aceita números duplicados, a pesquisa binária retorna sempre a PRIMEIRA
    // correspondência no vetor. Isso não quer dizer que o resultado é, na ordem do vetor, a
    // primeira vez que o valor aparece no vetor, apenas que o valor no indice retornado equivale
    // ao valor desejado. Se o objetivo fosse encontrar todos os valores correspondentes,
    // poderiamos fazer a pesquisa binária e a partir do valor retornado, procurar tanto nos
    // valores acima quanto nos valores abaixo até encontrar um valor que não fosse o valor
    // desejado e retornar essa distância de valores.
    for i in &vetor {
        println!("{i}");
    }
    println!();
    inserir_valor(&mut vetor, 15);
    for i in &vetor {
        println!("{i}");
    }
    println!();
    let resultado: i32 = pesquisa_binaria(&vetor, 5);
    println!("{resultado}");
}

// Gera um vetor com valores aleatórios a partir dos argumentos passados à função
fn gerar_vetor(tam: usize, max: u32) -> Vec<u32> {
    let mut vetor: Vec<u32> = Vec::new();
    while vetor.len() < tam {
        vetor.push(rand::rng().random_range(0..=max));
    }
    vetor
}

// Retorna o total de digitos de um valor númerico
fn contar_digitos(valor_original: u32) -> u32 {
    let mut n: u32 = 1;
    let mut contagem: u32 = 0;
    while n <= valor_original {
        contagem += 1;
        n *= 10;
    }
    contagem
}

fn counting_sort(alvo: &mut Vec<u32>, n: u32) {
    // Define um valor 10 vezes maior que a casa decimal atual. Esse valor será usado
    // em seguida para fazer operações com cada valor do vetor alvo para extrair o valor da casa
    // decimal atual.
    let digito = n * 10;

    // Define uma variável com o tamanho do vetor alvo para evitar repetir a mesma operação
    // diversas vezes
    let tamanho: usize = alvo.len();

    // Cria um vetor vazio com o mesmo tamanho do vetor alvo que será utilizado para transformar o
    // vetor alvo no final da execução da função
    let mut resultado: Vec<u32> = Vec::new();
    for _i in 0..tamanho {
        resultado.push(0);
    }

    // Cria um array de tamanho 10 e define todos valores do array para 0. Esse array será
    // utilizado para contar a frequência de cada digito no vetor alvo. Acredito que esse método
    // seja mais devagar do que simplesmente guardar cada valor do vetor alvo em um array do seu
    // digito correspondente, porém acredito que dessa forma o programa consome menos memória. A
    // implementação dessa etapa depende do objetivo e das restrições do programa.
    let mut frequencia: [u32; 10] = [0; 10];
    // Aqui iteramos para cada valor entre 0 e o tamanho do vetor inicial. Utilizaremos a váriavel
    // i para processar todos os valores do vetor
    for i in 0..tamanho {
        // A etapa seguinte cria uma váriavel "valor" de tipo usize utilizando um match statement
        // para lidar com qualquer erro caso não seja possível converter o digito em usize. Seria
        // possível utilizar .unwrap() para fazer isso automaticamente porém desse modo é mais
        // fácil lidar com erros ou fazer qualquer processamento to resultado "r" antes de
        // retorná-lo como "valor"
        let valor: usize = match ((alvo[i] % digito - alvo[i] % n) / n).try_into() {
            Ok(r) => r,
            Err(e) => panic!("Erro na definição de valor em counting_sort: {e}"),
        };
        // Aumentamos o valor correspondente no array frequência para cada digito correspondente
        frequencia[valor] += 1;
    }

    // Aqui ocorre uma soma de todos números de frequência resultando em um "alcance" de cada
    // digito. Esses valores serão utilizados em seguida
    for i in 1..10 {
        frequencia[i] += frequencia[i - 1];
    }

    // Iterando do tamanho do vetor inicial até 0, fazemos a mesma operação anterior para extrair o
    // digito de interesse do valor atual e posicionamos o valor no vetor de resultado a partir do
    // array de frequência de cada digito
    for i in (0..tamanho).rev() {
        let valor: usize = match ((alvo[i] % digito - alvo[i] % n) / n).try_into() {
            Ok(r) => r,
            Err(e) => panic!("Erro na definição de valor em counting_sort: {e}"),
        };
        resultado[(frequencia[valor] - 1) as usize] = alvo[i];
        frequencia[valor] -= 1;
    }
    *alvo = resultado;
}

// A função radix_sort vai ser apenas uma interface para executar a função counting_sort, pois um
// radix sort é apenas um conjunto de counting sorts executados para cada digito do maior número do
// vetor ou array. Além disso, não é necessário retornar nenhum valor pois a função vai alterar
// diretamente alterar o vetor original.
fn radix_sort(mut alvo: &mut Vec<u32>) {
    // Cria uma variável com o valor do primeiro número do vetor alvo
    let mut maior: u32 = alvo[0];
    // Compara essa variável com todos os valores do vetor para determinar o maior valor no vetor
    for i in 1..alvo.len() {
        if alvo[i] > maior { maior = alvo[i] };
    }

    // Cria uma variável com a quantidade de digitos do maior valor do vetor
    let i: u32 = contar_digitos(maior);

    // Executa a função counting_sort para cada casa decimal no maior valor do vetor
    let mut n: u32 = 1;
        while n < 10_u32.pow(i) {
        counting_sort(&mut alvo, n);
        n *= 10;
    }
}

// Ufa... Agora podemos começar o processo de fazer a busca binária para achar um elemento no vetor
// ordenado. Não vou complicar muito aqui, vou apenas traduzir o código em Python disponibilizado
// na outra atividade de hoje
fn pesquisa_binaria(vetor: &Vec<u32>, valor_desejado: u32) -> i32 {
    let tamanho: usize = vetor.len();
    let mut inicio: i32 = 0;
    let mut fim: i32 = (tamanho - 1) as i32;
    let mut meio: i32;
    // No algoritmo original também existe um contador de etapas utilizadas para a execução da
    // pesquisa binária. Não vou incluir essa funcionalidade aqui pois acredito que não seja
    // necessário.
    while inicio <= fim {
        meio = (inicio + fim) / 2;
        if vetor[meio as usize] == valor_desejado {
            return meio as i32;
        } else if vetor[meio as usize] < valor_desejado {
            inicio = meio + 1;
        } else {
            fim = meio - 1;
        }
    }
    return -1;
}

// Por fim, é necessário também desenvolver uma função para inserir um valor novo no vetor de forma
// ordenada. Para isso utilizarei um algoritmo de pesquisa linear que retorna o indice do o primeiro 
// valor maior que o desejado OU retorna o tamanho do vetor
fn pesquisa_linear(vetor: &Vec<u32>, valor: u32) -> u32 {
    let tamanho: usize = vetor.len();
    for i in 0..tamanho {
        if vetor[i] > valor { return (i) as u32 };
    }
    return tamanho as u32;
}

fn inserir_valor(vetor: &mut Vec<u32>, valor: u32) {
    let tamanho: usize = vetor.len();
    let indice: u32 = pesquisa_linear(&vetor, valor);
    // Primeiramente é necessário adicionar mais um valor ao vetor para aumentar o tamanho desse
    vetor.push(0);
    // Em seguida deslocamos todos os valores após o indice desejado para 1 indice a mais no vetor
    for i in (((indice + 1) as usize)..tamanho + 1).rev() {
        vetor[i] = vetor[i - 1];
    }
    // Por fim transformamos o indice descoberto na pesquisa linear no valor desejado
    vetor[indice as usize] = valor;
}
