# Temperature Converter (Rust)

## 📘 Descripción

**Temperature Converter** es un sencillo programa escrito en **Rust** que permite convertir temperaturas entre **Celsius** y **Fahrenheit**.  
El usuario puede elegir la dirección de la conversión a través de un pequeño menú interactivo en la terminal.

---

## ⚙️ Ejecución

1. Asegúrate de tener **Rust** instalado en tu sistema.  
   Puedes verificarlo con:
   ```bash
   rustc --version
   ```

2. Clona o descarga el proyecto y navega hasta el directorio donde se encuentra el archivo `main.rs`.

3. Ejecuta el programa utilizando **Cargo**:
   ```bash
   cargo run
   ```

---

## 🧩 Funcionamiento

Al ejecutar el programa, se mostrará el siguiente menú:

```
Temperature Converter
1- Celsius to Fahrenheit
2- Fahrenheit to Celsius
Please select an option (1 or 2):
```

El usuario debe introducir:
- `1` para convertir de **Celsius** a **Fahrenheit**.
- `2` para convertir de **Fahrenheit** a **Celsius**.

Después de seleccionar la opción, se pedirá introducir la temperatura a convertir.

El resultado se mostrará con **dos decimales**.

---

## 🧮 Ejemplo de uso

### Ejemplo 1: Celsius → Fahrenheit
```
Temperature Converter
1- Celsius to Fahrenheit
2- Fahrenheit to Celsius
Please select an option (1 or 2):
1
Enter temperature in Celsius:
25
25.00º Celsius is 77.00º Fahrenheit
```

### Ejemplo 2: Fahrenheit → Celsius
```
Temperature Converter
1- Celsius to Fahrenheit
2- Fahrenheit to Celsius
Please select an option (1 or 2):
2
Enter temperature in Fahrenheit:
100
100.00º Fahrenheit is 37.78º Celsius
```

---

## 🗂️ Estructura del proyecto

```
temperature_converter/
│
├── src/
│   └── main.rs
│
└── Cargo.toml
```

---

## 🧑‍💻 Autor

Desarrollado en **Rust** con fines educativos.  
Ideal para quienes se inician en la programación con Rust y desean practicar entrada/salida y control de flujo.

