# Simulador Dios

Un motor de simulación de alto rendimiento construido con Rust y enlaces Python usando Maturin.

## Estructura del Proyecto

```
simulador-dios/
├── README.md                 # Documentación del proyecto
├── rust_core/               # Biblioteca principal en Rust
│   ├── Cargo.toml          # Configuración del paquete Rust
│   └── src/
│       └── lib.rs          # Código principal de la biblioteca Rust
└── venv-rust/              # Entorno virtual de Python
    ├── pyvenv.cfg          # Configuración del entorno virtual
    ├── bin/                # Binarios ejecutables
    │   ├── python          # Intérprete de Python
    │   ├── python3         # Intérprete de Python 3
    │   ├── activate        # Script de activación del entorno
    │   ├── maturin         # Herramienta de construcción Maturin
    │   └── pip             # Instalador de paquetes
    ├── lib/                # Bibliotecas de Python (32-bit/compatibilidad)
    ├── lib64/              # Bibliotecas de Python (64-bit)
    └── include/            # Archivos de cabecera C para extensiones Python
```

## Descripción de Componentes

### Núcleo Rust (`rust_core/`)

El corazón del motor de simulación escrito en Rust para máximo rendimiento:

- **`Cargo.toml`**: Manifiesto del paquete Rust que define dependencias, metadatos y configuración de construcción
- **`src/lib.rs`**: Punto de entrada principal de la biblioteca que contiene la lógica de simulación central y enlaces Python

### Entorno Python (`venv-rust/`)

Un entorno virtual Python dedicado configurado para integración Rust-Python:

- **Maturin**: Herramienta de construcción que compila código Rust en wheels de Python
- **Enlaces Python**: Módulos Python generados que exponen funcionalidad Rust
- **Herramientas de Desarrollo**: pip, intérpretes Python y scripts de activación

## Tecnologías Clave

### Maturin
Maturin es el sistema de construcción que conecta Rust y Python:
- Compila código Rust en wheels compatibles con Python
- Soporta enlaces PyO3, CFFI y UniFFI
- Maneja empaquetado y distribución
- Proporciona soporte de backend de construcción PEP 517

### Integración Python
El proyecto utiliza el ecosistema de empaquetado de Python:
- Aislamiento del entorno virtual con `venv-rust`
- Gestión de paquetes a través de pip
- Compatibilidad multiplataforma

## Flujo de Desarrollo

## Instalación


1. **Activar el entorno virtual**:
   ```bash
   python3 -m venv venv-rust
   source venv-rust/bin/activate
   ```

2. **Compilar e instalar el modulo Rust**:
   ```bash
   cd rust_core
   maturin develop
   cd ..
   ```
3. **Compilar e instalar el modulo Rust**:
   ```bash
   python venv-rust/python_app/main.py
   ```


### Construir el Proyecto

1. **Activar el entorno virtual**:
   ```bash
   source venv-rust/bin/activate
   ```

2. **Construir la extensión Rust**:
   ```bash
   cd rust_core
   maturin develop
   cd..
   ```

3. **Construir wheels para distribución**:
   ```bash
   python venv-rust/python_app/main.py
   ```



## Uso

Una vez instalado, puedes usar el simulador desde Python:

```python
import simulador_dios

# Tu código de simulación aquí
```
