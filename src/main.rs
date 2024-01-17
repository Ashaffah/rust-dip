// Trait untuk mendefinisikan perilaku dari sebuah database
trait Database {
    fn connect(&self);
    fn query(&self, query: &str) -> Vec<String>;
}

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

fn main() {
    let mysql_database = MySQLDatabase;
    let postgresql_database = PostgreSQLDatabase;

    let mysql_data_manager = DataManager {
        database: mysql_database,
    };
    let postgresql_data_manager = DataManager::new(postgresql_database);

    let query = "SELECT * FROM users";

    let mysql_results = mysql_data_manager.perform_query(query);
    println!("Hasil MySQL: {:?}", mysql_results);

    let postgresql_results = postgresql_data_manager.perform_query(query);
    println!("Hasil PostgreSQL: {:?}", postgresql_results);
}

/*
 *   Dalam kode di atas, terdapat penerapan prinsip Dependency Inversion.
 *   Prinsip ini terlihat pada penggunaan trait Database sebagai abstraksi untuk menggambarkan perilaku sebuah database.
 *   Implementasi konkret dari MySQLDatabase dan PostgreSQLDatabase kemudian mengimplementasikan trait Database.
 *   DataManager juga menggunakan generic type parameter T yang terikat pada trait Database.
 *   Hal ini memungkinkan DataManager untuk bergantung pada abstraksi (trait Database) daripada implementasi konkret (MySQLDatabase atau PostgreSQLDatabase).
 *   Dengan demikian, DataManager tidak bergantung langsung pada implementasi database tertentu, melainkan hanya pada perilaku yang didefinisikan oleh trait Database.
 *   Ini memungkinkan fleksibilitas dalam mengganti implementasi database tanpa mengubah kode DataManager.
 */
