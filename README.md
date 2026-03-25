# Algoritmo de ordenamento RADIX e algoritmo de pesquisa binária em Rust
Esse repositório foi criado para compartilhar o código e a pesquisa relacionados a uma atividade proposta no curso de engenharia de software na Universidade do Vale do Taquari

## Por que Rust?
A atividade permitia utilizar qualquer linguagem de programação para realizar a proposta. Decidi utilizar Rust pois é a linguagem que eu estou atualmente aprendendo e acredito que é uma linguagem muito poderosa, tendo um gerenciamento de memória muito seguro sem comprometer na performance.

## Por que ordenamento RADIX?
A tabela a seguir foi utilizada como refêrencia para a escolha do algoritmo.
<img width="1314" height="1024" alt="image" src="https://github.com/user-attachments/assets/cd1adcd4-2f4f-4578-a85d-b7f2bd971f85" />
Em resumo, o algoritmo de ordenamento RADIX tem uma performance excelente na maioria das situações, além de ser complicado de implementar, permitindo desenvolver algumas habilidades de desenvolvimento de algoritmo.

## Pesquisa
Antes de começar o desenvolvimento do algoritmo, fui atrás de referências para compreender o funcionamento do algoritmo. Os principais sites utilizados foram o [W3Schools](https://www.w3schools.com/dsa/dsa_algo_radixsort.php), o [Baeldung](https://www.baeldung.com/java-radix-sort) e o [GeeksForGeeks](https://www.geeksforgeeks.org/dsa/counting-sort/). Além disso, os [slides do professor Luciano Antonio Digiampietri](https://www.each.usp.br/digiampietri/SIN5013/11-tempoLinear_RadixSort.pdf) e o [vídeo do CS Dojo](https://youtu.be/XiuSW_mEn7g?si=aW8DFC5vCoDxLguX) foram utilizados de refêrencia. Nenhum tipo de inteligência artificial foi utilizado na pesquisa e nem no desenvolvimento do código final.
Após a pesquisa, montei um fluxograma com as etapas necessárias para realizar o algoritmo e comecei a desenvolver. Utilizei o pacote rand para gerar números aleatórios para preencher o vetor inicial.
<img width="1112" height="1654" alt="documento-1" src="https://github.com/user-attachments/assets/58ec8b4b-8327-456f-8e6a-2ef7579ec0d5" />
Após a aplicação do algoritmo de ordenamento RADIX, o resto da atividade foi relativamente fácil. Utilizei os códigos disponibilizados pelo professor como refêrencia para o algoritmo de pesquisa binária e montei rapidamente o algoritmo de inserção usando um algoritmo de pesquisa linear modificado para retornar o índice de um valor maior que desejado, e não o valor desejado em si.

## Limitações
Pelo escopo da atividade, não implementei entrada de usuário e nem funções avançadas como processamento de um vetor com valores negativos. Esse é um algoritmo relativamente simples, com foco apenas na lógica por trás do ordenamento de um vetor e não na sua utilização com um algoritmo verdadeiro. Além disso, devido ao algoritmo selecionado achei que seria justo utilizar funções nativas apesar do que foi pedido na tarefa pelo fato de que o algoritmo já é complexo o suficiente e teria sido fácil implementar algoritmos para, por exemplo, extrair o tamanho de um vetor.
