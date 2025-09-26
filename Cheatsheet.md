# 🚀 Language Cheat Sheet

### 📦 Variables

```
let a = 10; // Number
let b = 10.5; // Float
let c = "Hello"; // String
let d = true; // Boolean
let e = nil; // Nil
let obj = { "key": "value" }; // Table
```

---

### 🔢 Operators

```
// Arithmetic
+  -  *  /

// String concat
"Hi " + "there"
10 + " apples"

// Ternary
cond ? x : y

// Comparison
== != < <= > >=

// Negation
-10
!false
```

---

### 🧱 Blocks & Scope

```
{
  let x = 10;
  {
    let y = 20;
  }
}
```

---

### 🔀 Control Flow

```
if (cond) { ... } else { ... }

while (i < 10) { ... }

for (let i = 0; i < 10; i = i + 1) { ... }
```

Logical operators:

```
a and b
a or b
```

---

### 🛠️ Functions

```
fn add(a, b) {
  return a + b;
}

println(add(2, 3));
```

Recursion:

```
fn fact(n) {
  return n <= 1 ? 1 : n * fact(n - 1);
}
```

Closures:

```
fn make_counter() {
  let c = 0;
  fn inc() { c = c + 1; return c; }
  return inc;
}
let counter = make_counter();
println(counter()); // 1
```

---

### 📦 Tables

```
let person = {
  "name": "Sid",
  "age": 22,
};

println(person["name"]);
person["name"] = "Roy";
```

Objects:

```
fn create_actor() {
  let a = { "health": 100 };
  fn damage(n) { a["health"] = a["health"] - n; }
  a["damage"] = damage;
  return a;
}
```

---

### 📚 Standard Library

```
// IO
print("Hi");
println("Hello");
let x = input();

// String
str(x); // to string

// Generic
len("Hello"); // 5
len({ "a": 1, "b": 2 }); // 2
```
