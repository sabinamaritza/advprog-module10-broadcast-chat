## Module 10 Reflection

#### Experiment 2.1: Original Code and How it Runs

![experiment-2.1](img/experiment-2.1.png)

Pada eksperimen ini dijalankan aplikasi broadcast chat menggunakan websocket asynchronous: satu server
dan tiga client secara bersamaan.

Setiap client dapat mengirim pesan ke server, kemudian server akan melakukan broadcast pesan 
tersebut ke seluruh client yang sedang terkoneksi.

Asynchronous programming cocok digunakan pada aplikasi chat karena banyak client dapat menunggu 
pesan secara bersamaan tanpa harus membuat thread baru untuk setiap koneksi.