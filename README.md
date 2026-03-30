# README

## O que o projeto se propõe a fazer

O projeto é um protótipo em JavaScript de uma linguagem chamada Graphol, executada no navegador.

A proposta central da linguagem é modelar tudo como "nodos" que trocam mensagens:

- Uma expressão é uma sequência onde o primeiro nodo recebe os nodos seguintes.
- O tipo efetivo do nodo surge a partir da primeira mensagem recebida.
- Strings concatenam.
- Números usam operações aritméticas.
- Blocos funcionam como unidades executáveis.
- Condicionais e execução assíncrona são tratados como combinações de nodos especiais.

Na prática, o sistema pega um código-fonte Graphol escrito em um `textarea`, compila esse código para JavaScript e depois executa o resultado em uma máquina virtual simples baseada em escopos, blocos e threads cooperativas.

## Como o sistema funciona ponta a ponta

1. Uma página HTML de demonstração carrega `import.js`.
2. `import.js` injeta todos os scripts do runtime e do compilador em ordem fixa.
3. Ao final da carga, `import.js` instancia duas globais:
   - `vm = new grapholVm()`
   - `gc = new grapholCompiler()`
4. O usuário escreve Graphol no `textarea`.
5. Ao clicar em compilar, `gc.parser(...)` transforma o código Graphol em uma lista de instruções JavaScript.
6. Ao clicar em executar, `vm.load(...)` carrega essas instruções e `vm.exec()` percorre os blocos, faz `eval(...)` linha a linha e movimenta o fluxo entre escopos, blocos e threads.

## Arquitetura geral

### 1. Interface e demonstrações

As páginas HTML são demos manuais da linguagem. Cada uma ensina uma parte da sintaxe e oferece três botões:

- Compilar e executar
- Compilar
- Executar

Elas compartilham o mesmo runtime e variam apenas no texto explicativo e no exemplo de código.

### 2. Loader

`PrototipoJS/import.js` controla a ordem de carga dos arquivos. Essa ordem é importante porque o projeto depende de funções globais e não usa empacotador, módulos ES nem CommonJS.

### 3. Compilador

`PrototipoJS/compiler/compiler.js` implementa o parser/compilador. Ele faz análise manual de caracteres, reconhece nodos reservados, strings, números, nomes, parênteses e blocos entre chaves.

O compilador não produz AST formal. Em vez disso, gera diretamente linhas de JavaScript em texto.

### 4. Runtime

O runtime principal está em:

- `PrototipoJS/vm/graphol.js`
- `PrototipoJS/vm/grapholvm.js`
- `PrototipoJS/vm/graphol/lang/...`
- `PrototipoJS/vm/graphol/command/...`

Ele fornece:

- Escopo com busca hierárquica de nodos
- Nodo genérico com estratégia dinâmica
- Tipos primitivos e compostos
- Mensagens especiais como `run`, `async` e `else`
- Comandos como `echo`, `input` e `if`
- Saída configurável entre `alert` e `console`

## Fluxo interno entre os módulos

### Fluxo de compilação

- `grapholCompiler.parser(...)` consome o texto fonte inteiro.
- `processaExpressao(...)` interpreta uma expressão Graphol como "receptor + mensagens".
- `processaNodo(...)` reconhece:
  - operadores aritméticos
  - operadores lógicos
  - operadores booleanos
  - strings
  - números
  - nomes de nodos
- Quando encontra `{ ... }`, o compilador gera um `strategy_Block`, associa a VM e guarda o escopo pai.
- O resultado final é um texto com instruções JavaScript separadas por quebra de linha.

### Fluxo de execução

- `grapholVm.load(...)` divide o código compilado em blocos.
- `grapholVm.exec()` cria a thread inicial e passa a executar o array `p_blocks`.
- Cada thread mantém:
  - `IR.BASE`
  - `IR.ADDR`
  - `IR.SCOPE`
  - pilha de retorno
- Cada linha compilada é executada com `eval(...)`.
- Quando um bloco é chamado, a VM empilha o contexto atual e entra em um novo escopo `CGraphol`.
- Quando um bloco termina, `self.callback()` restaura o frame anterior ou remove a thread.
- Se o bloco tiver recebido a mensagem `async`, a VM cria uma nova thread para ele.

### Fluxo de escopo e resolução de nomes

- `CGraphol` guarda os nodos do escopo atual.
- `find(...)` tenta resolver primeiro no escopo local e depois sobe para o pai.
- `get(...)` retorna o nodo existente ou cria um novo `Nodo`.
- Blocos recebem um `inbox` para entrada de dados, disponível como nome reservado no escopo do bloco.

### Fluxo de tipagem dinâmica

- `Nodo` começa sem estratégia.
- A primeira mensagem define sua estratégia via `strategy_Factory(...)`.
- Depois disso, todas as mensagens são encaminhadas para a estratégia escolhida.
- Isso explica o comportamento descrito nas demos: um mesmo nome pode se comportar como texto, número, bloco ou operador dependendo do primeiro valor recebido.

## Análise arquivo por arquivo

### Arquivos raiz

| Arquivo | Papel |
|---|---|
| `README` | Existe, mas está vazio. Não documenta o projeto. |
| `.gitignore` | Ignora apenas metadados de NetBeans em `nbproject/`. |

### Páginas de demonstração

| Arquivo | Papel |
|---|---|
| `PrototipoJS/graphol.html` | Demo de regras básicas: atribuição por mensagens, strings, números e expressões aninhadas. |
| `PrototipoJS/graphol2.html` | Demo de operações numéricas e uso de operadores como nodos. |
| `PrototipoJS/graphol3.html` | Demo de blocos e uso de `run` com `inbox`. |
| `PrototipoJS/graphol4.html` | Demo de condicionais com comparações, negação e operadores booleanos. |
| `PrototipoJS/graphol5.html` | Demo de execução assíncrona e troca de estratégia de saída para `console`. |
| `PrototipoJS/graphol6.html` | Página adicional muito parecida com `graphol4.html`; parece uma variação/duplicata de teste da demo de condicionais. |
| `PrototipoJS/import.js` | Carrega todos os scripts do projeto em ordem e instancia a VM e o compilador ao final. |

### Núcleo de escopo e VM

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol.js` | Define `CGraphol`, o ambiente de execução com tabela de nodos e lookup encadeado por escopo pai. Também registra comandos e mensagens embutidos. |
| `PrototipoJS/vm/grapholvm.js` | Define `grapholVm`, a máquina virtual que carrega código compilado, gerencia threads, pilha de retorno, saltos e execução linha a linha via `eval(...)`. |

### Base da linguagem

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol/lang/base.js` | Define `strategy_Null` e `strategy_Factory`, que escolhe a estratégia correta para cada valor recebido por um nodo. |
| `PrototipoJS/vm/graphol/lang/nodo.js` | Define `Nodo`, o invólucro genérico que delega comportamento para uma estratégia concreta. |

### Tipos e estratégias

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol/lang/types/number.js` | Implementa `strategy_Number`, com acumulação numérica e troca dinâmica de operador. |
| `PrototipoJS/vm/graphol/lang/types/string.js` | Implementa `strategy_String`, concatenando tudo que recebe. |
| `PrototipoJS/vm/graphol/lang/types/boolean.js` | Implementa `strategy_Boolean`, encapsulando um valor booleano. |
| `PrototipoJS/vm/graphol/lang/types/operator.js` | Implementa `strategy_Operator`, que acumula operações aritméticas sobre um valor interno. |
| `PrototipoJS/vm/graphol/lang/types/block.js` | Implementa `strategy_Block`, que representa blocos executáveis, recebe `run`/`async`, carrega `inbox` e chama a VM. |

### Operadores lógicos e booleanos

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol/lang/booleanOperators/logicOperator.js` | Implementa comparações como `==`, `!=`, `>`, `<`, `>=`, `<=` com validação de tipo entre operandos. |
| `PrototipoJS/vm/graphol/lang/booleanOperators/booleanOperator.js` | Implementa operadores booleanos `&&`, `||`, `!` e `x|` (xor). |

### Mensagens especiais

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol/lang/messages/run.js` | Define a mensagem `run`, usada para executar blocos. |
| `PrototipoJS/vm/graphol/lang/messages/async.js` | Define a mensagem `async`, usada para marcar blocos para nova thread. |
| `PrototipoJS/vm/graphol/lang/messages/else.js` | Define a mensagem `else`, usada pelo comando `if`. |

### Comandos

| Arquivo | Papel |
|---|---|
| `PrototipoJS/vm/graphol/lang/commands/if.js` | Implementa o comando `if`, consumindo pares condição/bloco e opcionalmente um `else`. |
| `PrototipoJS/vm/graphol/command/echo.js` | Implementa `echo`, enviando texto para a estratégia de saída atual. |
| `PrototipoJS/vm/graphol/command/input.js` | Implementa `input` com `prompt(...)`, retornando valor textual/numérico. |
| `PrototipoJS/vm/graphol/command/stdout/core.js` | Implementa `Stdout`, que seleciona entre `Alert` e `Console`. |
| `PrototipoJS/vm/graphol/command/stdout/alert.js` | Estratégia de saída baseada em `alert(...)`. |
| `PrototipoJS/vm/graphol/command/stdout/console.js` | Estratégia de saída baseada em `console.log(...)`. |

### Compilador

| Arquivo | Papel |
|---|---|
| `PrototipoJS/compiler/compiler.js` | Traduz o código Graphol para JavaScript executável pela VM. É o coração da sintaxe da linguagem. |

## Como os arquivos se comunicam

| Origem | Destino | Relação |
|---|---|---|
| `graphol*.html` | `import.js` | Todas as demos dependem do loader para trazer o runtime e o compilador. |
| `import.js` | todos os scripts do runtime | Define a ordem de carga e garante que símbolos globais existam antes do uso. |
| `import.js` | `grapholVm` e `grapholCompiler` | Instancia as globais `vm` e `gc`. |
| `grapholCompiler` | `strategy_Block` e `grapholVm` | Ao compilar blocos, gera código que cria blocos executáveis ligados a `self` (a VM atual). |
| `grapholVm` | `CGraphol` | Cada frame/thread recebe um escopo Graphol próprio. |
| `CGraphol` | comandos, mensagens e nodos dinâmicos | Expõe `input`, `echo`, `if`, `run`, `async`, `else`, `stdout` e nomes definidos pelo usuário. |
| `Nodo` | `strategy_Factory` | Delegação de tipagem dinâmica. |
| `strategy_Factory` | tipos concretos | Seleciona `strategy_String`, `strategy_Number`, `strategy_Boolean`, `strategy_Block` etc. |
| `If` | `Else` e blocos | Interpreta `else` como mensagem de controle para executar o bloco alternativo. |
| `strategy_Block` | `grapholVm.call(...)` | Inicia execução do bloco em thread atual ou nova thread. |
| `Echo` | `Stdout` | Envia a representação textual do valor para a saída atual. |
| `Stdout` | `Alert` ou `Console` | Troca a estratégia de exibição de resultado. |

## O que o projeto faz de fato hoje

Pelo estado atual do código, o projeto entrega:

- Um protótipo navegável da linguagem Graphol
- Um compilador direto de Graphol para JavaScript
- Um runtime com escopos e blocos
- Suporte a:
  - texto
  - número
  - operadores aritméticos
  - comparações
  - operadores booleanos
  - blocos
  - condicionais
  - entrada por `prompt`
  - saída por `alert` ou `console`
  - execução assíncrona simulada por múltiplas threads na VM

Ele não entrega:

- empacotamento moderno
- testes automatizados
- documentação formal
- separação entre parser, AST e codegen
- segurança de execução, porque depende de `eval(...)`

## Observações técnicas relevantes

- O projeto tem perfil claro de protótipo/experimento, não de produto pronto.
- O runtime depende fortemente de variáveis globais e ordem de carga manual.
- A VM executa o código compilado com `eval(...)`, o que simplifica o protótipo mas aumenta acoplamento e risco.
- Há métodos incompletos ou pouco usados, como `Nodo.equals(...)`, partes de `strategy_Null()` e `tonumber/tostring` vazios em `strategy_Block`.
- Há sinais de nomenclatura inconsistente para booleanos (`toBoolean` vs `toboolean`) em partes da base, sugerindo código experimental.
- `graphol6.html` aparenta ser redundante em relação a `graphol4.html`.
- O texto explicativo de algumas demos não acompanha exatamente a funcionalidade demonstrada, indicando documentação interna ainda imatura.

## Conclusão

Graphol é um protótipo de linguagem orientada a nodos e mensagens, implementado integralmente em JavaScript para navegador. O repositório concentra três camadas:

- demos HTML para experimentação manual
- um compilador textual que gera JavaScript
- uma VM simples que executa esse JavaScript com escopos, blocos e threads
