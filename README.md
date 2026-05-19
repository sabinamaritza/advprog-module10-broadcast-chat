## Module 10 Reflection

#### Experiment 2.1: Original Code and How it Runs

![experiment-2.1](img/experiment-2.1.png)

Pada eksperimen ini dijalankan aplikasi broadcast chat menggunakan websocket asynchronous: satu server
dan tiga client secara bersamaan.

Setiap client dapat mengirim pesan ke server, kemudian server akan melakukan broadcast pesan 
tersebut ke seluruh client yang sedang terkoneksi.

Asynchronous programming cocok digunakan pada aplikasi chat karena banyak client dapat menunggu 
pesan secara bersamaan tanpa harus membuat thread baru untuk setiap koneksi.

<br>

#### Experiment 2.2: Modifying the Websocket Port

![experiment-2.2](img/experiment-2.2.png)

Pada eksperimen ini dilakukan perubahan websocket port menjadi 8080 pada sisi server dan client.

Kedua sisi harus menggunakan port yang sama agar websocket connection dapat berhasil dilakukan.

Selain itu, websocket protocol yang digunakan tetap sama yaitu `ws://`

Perubahan port perlu dilakukan pada URL websocket server yang digunakan client dan juga binding
address pada server. Eksperimen ini tidak mengubah pengiriman pesan antar client.

<br>

#### Experiment 2.3: Small Changes - Add IP and Port

![experiment-2.3](img/experiment-2.3.png)

Pada eksperimen ini di tambahkan informasi IP dan port pengirim pada setiap pesan chat.

Modifikasi dilakukan agar setiap client dapat mengetahui siapa pengirim pesan tersebut. Informasi
IP dan port diperoleh dari address koneksi websocket masing-masing client.

Ketika client mengirim pesan, server menerima pesan tersebut lalu menambahkan informasi address 
pengirim sebelum melakukan broadcast ke seluruh client lain.

Hal ini membantu memahami bagaimana data dikirim dan diteruskan dalam sistem asynchronous websocket.