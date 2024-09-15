# Language showcase
Here is a small showcase with explanation of each language feauter like:
- variables
- blocks
- ifs
- loops
- functions
Example programs can be found in (this folder)[/examples]


## Supported types
- strings
- booleans
- numbers (all numbers are treated as f64)
- nil
### Example
```
// booleans
var b1 = true;
var b2 = false;

// strings
var s1 = "string example";

// numbers
var n1 = 2;
var n2 = 2.3;

// nil
var n = nil;

```

---

## Print
print is a built in special keyword that allows user to print to standard output
- can be parsed only as a statement
### Pattern
```
print <expression>;
```
### Example
```
print "Hello, world!";
```

---

## Arithmetics
mathematical and boolean expressions are supported.

Supported math operators: `+`, `-`, `*`, `/`

Supported boolean operators: `and`, `or`

Supported comparision operators: `==`, `!=`, `<`, `>`, `<=`, `>=`

Also language support two main unary operators: `-`, `!`

### Examples
```
1 + 2 * 3 // evaluates to `7.0`
```
```
3 * 4 < 15 // evaluates to `true`
```
```
!(1 < 2) // evaluates to `false`
```

## Var
lets user to declare variables
- can be parsed only as a statement
### Pattern
```
var <ident> = <expression>;
```
```
var <ident>; // here the value is `nil`
```
### Example
```
var test = "test string";
```

---

## Block
lets user to define scoped set of statements
- can be parsed as statement or expression
- last non statement value is parsed as return expression
### Pattern
```
{ <set_of_statements> <return_expression> }
```
`return_expression` is optional here if the block ends with an expression it becomes its return value. It also works fo `fn` blocks if the funtion does not return early it returns the default block return value so either the last expression or `nil`
### Example
```
{
    var test = "test string";
}
var block_value = {
    1 + 2 + 3
};

```


---

## If
- can be parsed as statement or expression
- its block can have return value
- returns value of a block
- can have else statement
### Pattern
```
if(<condition_expression>) <yes_statement> else <no_statement>
```
yes and no expressions should be blocks or have trailing semicolon to be treated as statements
### Example
```
if (1 < 2) print "yes"; else print "no";

var if_value = if (1 < 2) {
    1
} else {
    2
};
```


---

## For & While loops
- can be parsed as statement or expression
- its block cant have return value
- can return value by `break <expression>;`
- can be stopped with `break;`
- can be continued with `continue`
- returns `nil` by default
### Pattern
```
for(<init_statement>;<condition_expression>;<step_statement>) <loop_statement>
```
```
while(<condition_expression>) <loop_statement>
```
### Example
```
for(var i = 1; i < 3; i = i + 1) print i;

var for_val = for(var i = 1; i < 3; i = i + 1) {
    if (i == 2) {
        break i;
    }
};

```

---

## Fn
TBD