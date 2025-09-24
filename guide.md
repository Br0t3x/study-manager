# Utilização do _study_manager_
O study_manager é um projeto em formato CLI (Interface de linha de comando)
que serve para gerenciar seus estudos, seja via livros didáticos, ou um curso,
online ou não. Com ele você pode categorizar cada assunto, capítulo ou
qualquer outro sistema de partições que seus estudos envolvam, e, ainda,
registrar seu progresso.

Exemplo de uso: estudando matemática pela coleção Fundamentos de
Matemática Elementar. Cada livro da coleção (total de 11) é registrado
e tem todos os seus capítulos documentados no study_manager. Após isso,
é possível registrar ainda todos os exercícios de cada capítulo na aplicação
para controlar exatamente o progresso dos estudos.

Mas o study_manager faz só isso? Não!
O study_manager oferece ainda: anotações para cada exercício, capítulo ou
matéria; pontuação de aprendizado para cada uma das subpartições citadas;
relatório do andamento dos estudos; estatísticas legais e úteis para te
informar do seu aprendizado; calendário de aprendizado com opcionalmente,
notificações diretamente em seu desktop (e, quem sabe, futuramente
vinculação com algum serviço de calendário como Google Calendar, Notion ou
Samsung Calendar).

E ainda há mais que o study_manager faz! Ele é uma ferramenta simples, rápida
e leve, mas poderosa para te oferecer rapidez e simplicidade ao lidar com
esta tarefa estressante que é manter uma rotina de estudos e organizá-la.

## Criação de assuntos
Utilize `new "<nome do assunto>"` (note que as aspas são facultativas para
nomes que possuam apenas uma palavra) para criar um assunto.

## Entendendo módulos (subpartições) de assuntos
Esse recurso pode ser utilizado de duas maneiras principais: com classificação
orientada por uma etiqueta (_label_) ou sem classificação. Você pode utilizar
ambas em conjunto se desejar.

Abaixo segue um cenário contextualizador que lhe ajudará a entender melhor o
uso de ambos:

Suponhamos que você está estudando matemática do ensino médio via livros
didáticos. Algumas coleções são organizadas em três volumes, e outras, como
_Fundamentos de Matemática Elementar_, __onze__ volumes.

<details>
<summary>Quando utilizar um módulo sem classificação</summary>

Quando for mais conveniente criar novos __assuntos__ para guardar conteúdo
de estudo ao invés de novos _módulos_.
Usando como base o exemplo, essa opção é adequada para quando você estuda um
assunto dividido em apenas três volumes.
É assim, preferível separar os volumes em diferentes assuntos (um total de 3),
tal que os módulos sejam capítulos dos volumes. Essa solução mantém
encapsulado o conteúdo de cada volume em um único assunto, permitindo um
controle mais fino do estudo de cada volume.
</details>
<details>
<summary>Quando utilizar um módulo com etiqueta?</summary>

Quando for mais conveniente condensar todos os livros em uma única
matéria/assunto. Usando o exemplo como base, você pode estar estudando com
a coleção _Fundamentos de Matemática Elementar_, que contém 11 volumes.
Cada volume possui ao menos 3 capítulos (alguns mais que 10). Imagine
como seria cansativo criar 11 assuntos/matérias diferentes para cada um dos
volumes. Diante isso, você pode condensar todos os capítulos de todos os 
volumes numa única __supermatéria__. Para classificar cada capítulo como 
pertencente a um volume específico da coleção, basta que utilize uma
etiqueta no capítulo (chamado de módulo aqui). Desse modo, você pode trabalhar
individualmente cada capítulo ou volume, mesmo que estejam todos em um único
assunto/matéria.
</details>

## Criando módulos
### Uso básico
A estrutura sintática básica para a criação de um módulo é a seguinte:
- `new module "<nome do módulo>"`: cria um novo módulo no assunto
atualmente selecionado.
- `new module "<nome do módulo>" --label "<nome da etiqueta>"`: cria um novo
módulo etiquetado no assunto atualmente selecionado.

Para um controle mais fino dos módulos, você pode também criar módulos
em assuntos não atualmente selecionados por meio da seguinte sintaxe:
- `new module:<identidade do assunto> "<nome do módulo>"`: cria um novo módulo
no assunto identificado em `<identidade do assunto>`.
- `new module:<identidade do assunto> "<nome do módulo>" --label "<etiqueta>"`: cria um novo módulo
etiquetado no assunto especificado em `<identidade do assunto>`.

Mas e se for necessário criar vários módulos de uma vez (etiquetados ou não)? __Praticidade__ é
regra no study_manager, logo, há como fazer isso de duas principais formas: 
referenciando um arquivo com os nomes dos módulos ou listando-os diretamente no comando:
- `new module "<caminho relativo para o arquivo>"` ou `new module:<identidade do assunto> <caminho
relativo para o arquivo>`: cria vários módulos lendo individualmente as linhas de um arquivo.
- `new module <módulo1>; <módulo2>; ...; <móduloN>` ou `new module:<identidade do assunto>
<módulo1>; <módulo2>; ...; <móduloN>`: cria vários módulos com base na lista. Lembre que para
módulos com <u>nomes compostos</u>, você deve envolver entre aspas o nome do módulo. Cada módulo deve
ser separado por ponto e vírgula.

__PS__: para etiquetar os módulos todos de uma vez, basta adicionar `--label "<nome da etiqueta>"`
no fim do comando. Para atribuir diferentes etiquetas de uma única vez, você deve listá-las
no mesmo formato que os módulos: `--label <Etiqueta1>; <Etiqueta2>; ...; <Etiqueta>`.
__PS 2__: Ao atribuir várias etiquetas de uma vez, você estará atribuindo as etiquetas listadas
para cada módulo informado, ou seja, as etiquetas serão aplicadas em todos os módulos, e não
no formato "etiqueta x para módulo x".

### Criando lista de módulos
Para criar vários módulos a partir de um arquivo, você deve ter um arquivo onde cada linha possui
o seguinte formato: `<Nome do  módulo>: <Etiqueta1>; <Etiqueta2>`.

__Nota__: é facultativo o uso de extensão no nome do arquivo.

Exemplo:

\* ./modulos

```
Noções de lógica: Volume 1; Introdução
Introdução às funções: Função Afim: Volume 1
Progressões: aritmética e geométrica:
```
__Observação__: note que nas linhas 2 e 3 há duas presenças de `:`. Quando seu módulo possuir esse
caractere no nome, para evitar que ele seja considerado como um separador entre o nome do módulo e
as etiquetas, você deve utilizá-lo novamente no final da linha, mesmo que nada seja listado após.
Use a linha 3 como exemplo disso.

Aqui, é importante notar também que é dispensável o uso de
aspas ao redor de nomes com mais de uma palavra.

