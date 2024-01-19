# rust-dip

Contoh Rust Dependency Inversion Principle (DIP)

## Dependency Injection

Dependency Injection adalah teknik di mana objek hanya menyediakan dependencies (ketergantungan) melalui konstruktor, metode, atau langsung ke properti. Dalam project ini, `DataManager` menerima sebuah implementasi dari trait `Database` sebagai dependency.

Berikut adalah penjelasan lebih lanjut:

1. **Abstraksi dengan Trait**: Trait `Database` digunakan sebagai abstraksi untuk menggambarkan perilaku sebuah database. Ini memungkinkan kita untuk mendefinisikan metode yang harus diimplementasikan oleh setiap jenis database.

```rust
// Trait untuk mendefinisikan perilaku dari sebuah database
trait Database {
    fn connect(&self);
    fn query(&self, query: &str) -> Vec<String>;
}
```

2. **Implementasi Konkret**: `MySQLDatabase` dan `PostgreSQLDatabase` merupakan implementasi konkret dari sifat `Database`. Mereka mendefinisikan bagaimana operasi basis data dilakukan untuk basis data MySQL dan PostgreSQL.

```rust
// Implementasi konkret dari trait Database untuk MySQL
struct MySQLDatabase;

impl Database for MySQLDatabase {
    fn connect(&self) {
        println!("Menghubungkan ke database MySQL...");
    }

    fn query(&self, query: &str) -> Vec<String> {
        println!("Menjalankan query: {}", query);
        // Melakukan query sebenarnya dan mengembalikan hasilnya
        vec!["Hasil 1".to_string(), "Hasil 2".to_string()]
    }
}

// Implementasi konkret dari trait Database untuk PostgreSQL
struct PostgreSQLDatabase;

impl Database for PostgreSQLDatabase {
    fn connect(&self) {
        println!("Menghubungkan ke database PostgreSQL...");
    }

    fn query(&self, query: &str) -> Vec<String> {
        println!("Menjalankan query: {}", query);
        // Melakukan query sebenarnya dan mengembalikan hasilnya
        vec!["Hasil 3".to_string(), "Hasil 4".to_string()]
    }
}
```

3. **Penggunaan Generic Type Parameter**: `DataManager` menggunakan parameter tipe umum `T` yang terikat dengan sifat `Database`. Ini berarti bahwa `DataManager` dapat bekerja dengan semua jenis database yang mengimplementasikan sifat `Database`.

```rust
// Modul tingkat tinggi yang bergantung pada trait Database
struct DataManager<T: Database> {
    database: T,
}

impl<T: Database> DataManager<T> {
    fn new(database: T) -> Self {
        DataManager { database }
    }

    fn perform_query(&self, query: &str) -> Vec<String> {
        self.database.connect();
        self.database.query(query)
    }
}

```

4. **Dependency Injection**: `DataManager` tidak membuat instance dari `MySQLDatabase` atau `PostgreSQLDatabase` secara langsung. Sebaliknya, sebuah instance dari `Database` (baik `MySQLDatabase` atau `PostgreSQLDatabase`) diinjeksikan ke dalam `DataManager`. Ini berarti bahwa `DataManager` tidak bergantung secara langsung pada implementasi database tertentu, tetapi hanya pada perilaku yang ditentukan oleh properti `Database`.

```rust
let mysql_database = MySQLDatabase;
let mysql_data_manager = DataManager {
        database: mysql_database,
    }; // bisa juga ditulis DataManager::new(mysql_database);
```

5. **Fleksibilitas**: Dengan ini, kita dapat dengan mudah mengganti implementasi database tanpa mengubah kode `DataManager`. Sebagai contoh, jika kita ingin beralih dari MySQL ke PostgreSQL, kita hanya perlu mengubah instance `Database` yang diinjeksikan ke dalam `DataManager`.

```rust
let postgresql_database = PostgreSQLDatabase;
let postgresql_data_manager = DataManager::new(postgresql_database); // bisa juga ditulis DataManager { database: postgresql_database };
```
