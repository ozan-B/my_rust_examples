# 2- Multi-threaded attack surface discovery
# (Çok iş parçacıklı saldırı yüzeyi keşfi)

“Düşmanını tanımak için Düşmanın olmalısın”, Sun Tzu

Her saldırının ilk adımı keşiftir. Bu aşamanın amacı hedefimiz hakkında mümkün olduğunca çok bilgi toplamak ve bu sayede
yaklaşan saldırı.

Bu bölümde, keşif yapmanın temellerini ve kendi tarayıcımızı nasıl uygulayacağımızı göreceğiz, **ve çoklu iş parçacığından yararlanarak nasıl hızlandırılacağı.**

Keşif yapmanın iki yolu vardır: **Pasif** ve **Aktif**.

![r10](/img_rust/r10.png)

---

## 2.1 Passive reconnaissance

Pasif keşif, bir hedefle doğrudan etkileşime girmeden hedef hakkında bilgi toplama sürecidir, örneğin hedefi farklı sosyal ağlarda aramak ve arama motorları.

Halka açık kaynakların kullanılmasına Açık Kaynak İstihbaratı anlamına gelen OSINT denir.


**Pasif keşif kullanılarak ne tür veriler toplanır?**
- Bir şirketin çalışanları hakkında isimler, e-posta adresleri, telefon numaraları gibi bilgilerin yanı sıra
kaynak kod depoları.

- **[Shodan](https://www.shodan.io/)** gibi arama motorları sayesinde dünyaya açık hizmetler ve makineler arayın.

- **Pasif keşif 5. bölümün konusu olduğundan, dikkatimizi aktif keşfe odaklayacağız.**

---

## 2.2 Active reconnaissance

Aktif keşif, bir hedefle doğrudan etkileşime girerek hedef hakkında bilgi toplama sürecidir.

- Aktif keşif daha gürültülüdür ve güvenlik duvarları ve honeypotlar tarafından tespit edilebilir, bu nedenle
örneğin taramayı geniş bir zamana yayarak tespit edilmemeye dikkat etmelidir.

Honey Pot , belirli bir sistemdeki “normal” kişiler tarafından asla kullanılmaması gereken harici bir uç noktadır**(endpoint)**.

Şirket, bu nedenle bu uç noktaya yalnızca saldırganlar ulaşabilir. Bir posta sunucusu olabilir, bir HTTP sunucusu veya hatta uzak içerik gömülü bir belge. Bir bal küpü tarandığında veya vurulduğunda, onu yerleştiren güvenlik ekibine geri rapor verecektir.


- **[Thinkst Canary](https://github.com/thinkst/opencanary)**, bal küpü gibidir ancak dahili bir ağdadır. **Amacı saldırganları bir kez tespit etmektir** dış çevreyi ihlal etmişlerdir.

**Bir hedefin keşfi kendi içinde iki adıma ayrılabilir:**
  - Varlıkların keşfi
  - Güvenlik açıklarının belirlenmesi **(6. bölümün konusudur)**
  
---

## 2.3 Assets discovery

Geleneksel olarak varlıklar yalnızca teknik unsurlarla tanımlanırdı: **IP adresleri**, **sunucular**, **domain names**, **networks**...

Günümüzde kapsam(scope) daha geniştir ve **sosyal ağ hesaplarını**, **kamuya açık kaynak kodunu
depoları**, **Nesnelerin İnterneti nesneleri**... Günümüzde her şey internette veya internete bağlı
İnternet. Saldırgan bir bakış açısıyla, bu gerçekten ilginç.

**Bir hedefin tüm varlıklarını listelemenin ve haritalandırmanın amacı, yaklaşan saldırımız için giriş noktaları ve güvenlik açıkları bulmaktır**.


---

## 2.3.1 Subdomain enumeration

Kamu varlıklarının ortaya çıkarılmasına ilişkin en az çabayla en iyi sonuçları veren yöntem
alt alan adlarının numaralandırılması.

Gerçekten de, günümüzde bulut hizmetlerinin yaygınlaşmasıyla birlikte, giderek daha fazla şirket artık özel hizmetlerine erişmek için bir **VPN** gerektirir. **HTTPS aracılığıyla herkese açıktırlar.**

subdomain adlarının en erişilebilir kaynağı sertifika şeffaflık günlükleridir **([certificate transparency logs](https://certificate.transparency.dev/howctworks/))**. 


- **Ne zaman bir Sertifika Yetkilisi (CA)** bir web sertifikası yayınlar (örneğin HTTPS trafiğinde kullanım için). sertifikalar **herkese açık, şeffaf günlüklere kaydedilir.**


Bu günlüklerin meşru kullanımı, aşağıdaki sertifikaları verebilecek sahte sertifika yetkililerini tespit etmektir
sertifikaların yanlış varlıklara **(.google.com için bir sertifikanın teslim edildiğini hayal edin kötü niyetli bir bilgisayar korsanlığı ekibinin eline geçerse, bu onların Man In The Middle tespit edilmeden Google alan adları).**


Öte yandan, bu şeffaflık işimizin büyük bir bölümünü otomatikleştirmemizi sağlıyor.

- Örneğin, **kerkour.com** ve alt alanları için verilen tüm sertifikaları aramak için,
**https://crt.sh** adresine gidin ve **%.kerkour.com** adresini arayın **(% joker karakterdir)**:
**https://crt.sh/?q=%25.kerkour.com.**


Bu tekniğin bir sınırlaması, HTTP(S) olmayan hizmetleri (e-posta gibi) bulamamasıdır.
veya VPN sunucuları) ve joker alt alan adları (örneğin *.kerkour.com) olabilir.
gerçekte kullanılan alt alan adlarını gizlemek.


>`Bir anekdot olarak, şimdiye kadar gerçekleştirdiğim en hızlı güvenlik denetimi, bir şirketin GitLab örneği, tüm dünyaya açık kayıt ile halka açık olarak erişilebilir. GitLab'ı buldum
temel alt alan adı numaralandırma ile örnek. Bir hesap oluşturduğumda, şunlara erişimim oldu
Şirketin tüm (özel) kod depoları ve bunların birçoğu sırlar içeriyordu ve
bulut belirteçleri, şirketin tam olarak ele geçirilmesine yol açabilecek kodda işlendi
altyapı.`

---

## 2.3.1.1 What can be found

İşte alt alan adları taranarak bulunabileceklerin kapsamlı olmayan bir listesi:
- Kod depoları
- Devralmaya tabi unutulmuş alt alan adı
- Yönetici panelleri
- Paylaşılan dosyalar
- Depolama kovaları **(Storage buckets)**
- E-posta / Sohbet sunucuları

---

## 2.4 Our first scanner in Rust

Saldırı yüzeylerini haritalamak için kullanılan yazılımlara tarayıcı denir. **Port tarayıcısı**, **güvenlik açığı tarayıcısı**,**alt alan adı tarayıcısı**, **SQL enjeksiyon tarayıcısı**... Uzun ve titiz bir görevi otomatikleştirirler.



Ancak, tarayıcıların her derde deva olmadığını aklınızda bulundurmalısınız: çok gürültülü olabilirler ve Böylece niyetinizi ortaya çıkarabilir, **anti-spam sistemleri** tarafından engellenebilir veya eksik veri rapor edebilir.


**Amacı bir hedefin alt alan adlarını bulmak olan basit bir tarayıcıyla başlayacağız ve ardından her alt alan adı için en yaygın bağlantı noktalarını tarayacaktır.** 

Programlarımız giderek daha karmaşık hale geldikçe, öncelikle Rust'ta hata işleme anlayışımızı derinleştirmemiz gerekiyor.

---

## 2.5 Error handling

İster kütüphaneler ister uygulamalar için olsun, Rust'taki hatalar güçlü bir şekilde tiplendirilmiştir ve çoğu kütüphanemiz veya programımızdaki her hata türü için bir varyant içeren enumlar olarak karşılaşabilir.

Kütüphaneler için mevcut iyi uygulama **[thiserror](https://crates.io/crates/thiserror)** crate'ini kullanmaktır.

Programlar için ise **[anyhow](https://crates.io/crates/anyhow)** crate önerilmektedir. `Main` fonksiyonu  Tarafından döndürülen hataları güzelleştirecektir


![r11](/img_rust/r11.png)

---

## 2.6 Enumerating subdomains

**[crt.sh](https://crt.sh/)** tarafından sağlanan API'yi kullanacağız, bu API aşağıdaki şekilde sorgulanabilir

- **endpoint:** **https://crt.sh/?q=%25.[domain.com]&output=json**”.

![r12](/img_rust/r12.png)

- Şuna dikkat edin. Anlamı: “Çağrılan işlev bir hata döndürürse, geçerli işlevi iptal et
işlevini kullanın ve hatayı döndürün”.

---

## 2.7 Scanning ports

Alt alan adları ve IP adresleri numaralandırma, varlık keşfinin yalnızca bir parçasıdır. **Bir sonraki port taramasıdır**: hangi sunucuların herkese açık olduğunu keşfettikten sonra Bu sunucularda hangi hizmetlerin herkese açık olduğunu öğrenin.

**en basit tekniği kullanacağız:**

- bir TCP soketi açmaya çalışmak. Bu teknik TCP bağlantısı olarak bilinir çünkü bir TCP portuna bağlantı kurmaya çalışmaktan ibarettir. Soket bir tür internet borusudur. Örneğin, bir web sitesine bağlanmak istediğinizde,tarayıcınız web sitesinin sunucusuna bir soket açar ve ardından tüm veriler bu soket üzerinden geçer soket. 

- Bir soketin açık olması, sunucunun bağlantıları kabul etmeye hazır olduğu anlamına gelir. Açık Öte yandan, sunucu bağlantıları kabul etmeyi reddederse, bu hiçbir hizmetin olmadığı anlamına gelir. verilen bağlantı noktasını dinliyor. Bu durumda, bir zaman aşımı kullanmak önemlidir. Aksi takdirde, tarayıcımız takılı kalabilir (neredeyse) güvenlik duvarları tarafından engellenen bağlantı noktalarını tararken süresiz olarak.




![r13](/img_rust/r13.png)
![r14](/img_rust/r14.png)


Ancak bir sorunumuz var. Tüm isteklerimizi sıralı bir şekilde ateşlemek son derece yavaştır: eğer tüm

portlar kapatıldığında, `Number_of_scanned_ports * timeout` saniye bekleyeceğiz.

---

## 2.8 Multithreading

Neyse ki, programları hızlandırmak için bir API var: **Threads**. Thread, İşletim Sistemi (OS) tarafından sağlanan ve programcıların aşağıdakileri yapmasını sağlayan ilkel araçlardır **CPU'nun donanım çekirdeklerini ve iş parçacıklarını kullanır.** Rust'ta, bir iş parçacığı şu komut kullanılarak başlatılabilir
**`std::thread::spawn`** fonksiyonu.

![r15](/img_rust/r15.png)

Her bir CPU iş parçacığı bağımsız bir işçi olarak görülebilir: iş yükü aşağıdakiler arasında bölünebilir işçiler.Bu özellikle önemlidir, çünkü günümüzde fizik kanunları nedeniyle işlemciler **saniyedeki işlem sayısı (GHz)** açısından ölçeklendirme. 


Bunun yerine, satıcılar çekirdekler ve iş parçacıkları. Geliştiriciler, programlarını iş yükünü bölecek şekilde uyarlamalı ve tasarlamalıdır tüm işlemleri tek bir iş parçacığı üzerinde yapmaya çalışmak yerine, mevcut iş parçacıkları arasında er ya da geç işlemcinin sınırına ulaşabilirler.

İş parçacıkları ile büyük bir görevi paralel olarak yürütülebilecek daha küçük alt görevlere bölebiliriz.**Bizim durumumuzda, taranacak her bağlantı noktası için bir görev göndereceğiz**.

**Böylece, taranacak 100 portumuz varsa,100 görev oluşturacağız**.Tüm bu görevleri daha önce yaptığımız gibi sırayla çalıştırmak yerine birden fazla iş parçacığı üzerinde. **10 iş parçacığımız varsa, 3 saniyelik bir zaman aşımı ile, 30 saniyeye kadar ( 10 * 3 ) sürebilir.** tek bir ana bilgisayar için tüm bağlantı noktalarını tarar. **Bu sayıyı 100 iş parçacığına çıkarırsak, o zaman 100 portu sadece 3 saniyede tarayabilmelidir.**


---

## 2.9 Fearless concurrency in Rust

Malesef iş parçacığı kullanmak bedava ve kolay bir kazanç değildir. Eşzamanlılık **(concurency)** sorunları pek çok geliştiricinin korkusudur. Öngörülemeyen davranışları nedeniyle,tespit edilmesi ve hata ayıklanması son derece zordur. 

Uzun süre fark edilmeyebilirler ve sonra bir gün, sadece sisteminiz saniyede daha fazla istek işlediği için veya CPU'nuzu yükselttiğinizde, uygulamanız garip davranmaya başlar. 

**Nedeni neredeyse her zaman kod tabanınızda bir eşzamanlılık hatası gizlidir.** Rust ile ilgili en muhteşem şeylerden biri, sahiplik sistemi sayesinde derleyici, programlarımızın veri yarışı içermemesini garanti eder. **Örneğin, bir vektörü iki farklı iş parçacığında (kabaca) aynı anda değiştirmeye çalıştığımızda:**

![r16](/img_rust/r16.png)
![r17](/img_rust/r17.png)
![r18](/img_rust/r18.png)

Ne kadar uğraşırsak uğraşalım, derleyici veri yarışlarıyla kod derlememize izin vermeyecektir.

---

## 2.10 The three causes of data races ( Veri yarışlarının üç nedeni)

- İki veya daha fazla işaretçi aynı veriye aynı anda erişiyor.
- İşaretçilerden en az biri veriye yazmak için kullanılıyor.
- Verilere erişimi senkronize etmek için kullanılan bir mekanizma yok


----

## 2.11 The three rules of ownership (Mülkiyetin üç kuralı)

Rust'ta her değerin sahibi olarak adlandırılan bir değişkeni vardır.
- Aynı anda yalnızca bir sahip olabilir.
- Sahip kapsam dışına çıktığında, değer düşecektir.

---

## 2.12 The two rules of references (İki referans kuralı)

- Herhangi bir zamanda, tek bir değiştirilebilir referansa ya da istediğiniz sayıda değiştirilemez referansa sahip olabilirsiniz.
- Referanslar her zaman geçerli olmalıdır.

Bu kurallar son derece önemlidir ve Rust'ın bellek güvenliğinin temellerini oluşturur.


---

## 2.13 Other concurrency problems (Diğer eşzamanlılık sorunları)

Diğer eşzamanlılık sorunları Veri yarışları tek eşzamanlılık hatası değildir, aynı zamanda **[deadlock](https://en.wikipedia.org/wiki/Deadlock)** ve **[race conditions](https://en.wikipedia.org/wiki/Race_condition)** da vardır.


---

## 2.14 Adding multithreading to our scanner

Şimdi multithreading'in teoride ne olduğunu gördük. Şimdi de bunu Rust dilinde nasıl yapacağımızı görelim.

Genellikle, multithreading geliştiriciler tarafından korkulur çünkü az önce gördüğümüz hataları ortaya çıkarma olasılığı yüksektir.

**Ancak Rust'ta bu başka bir hikaye. Uzun süreli arka plan işleri veya thread başlatmak dışında, standart kütüphaneden thread API'sini doğrudan kullanmak nadirdir.**

Bunun yerine, Rust için bir veri paralelliği kütüphanesi olan **[rayon](https://github.com/rayon-rs/rayon)**'u kullanıyoruz.

Neden bir veri-paralelliği kütüphanesi? 

Çünkü iş parçacığı senkronizasyonu zordur. Programlarımızı iş parçacıklarının senkronize edilmesini gerektirmeyen işlevsel bir şekilde tasarlamak daha iyidir.

![r19](/img_rust/r19.png)


Ve... Hepsi bu. Gerçekten. **into_iter()** işlevini **into_par_iter()** (“paralel yineleyiciye” anlamına gelir) **ile değiştirdik.** 

**Yineleyici nedir? Daha fazlası bölüm 3'te**