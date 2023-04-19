use iced::widget::Text;

use crate::translations::types::language::Language;

pub fn choose_adapters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::DE => "Wähle einen Netzwerkadapter zum inspizieren aus",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
        Language::FA => "مبدل شبکه را برای بازرسی انتخاب کنید",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::KO => "검사할 네트워크 어댑터 선택",
        Language::PL => "Wybierz adapter sieciowy do inspekcji",
        Language::PT => "Selecione o adaptador de rede a inspecionar",
        Language::RO => "Selectați adaptor de rețea pentru a inspecta",
        Language::RU => "Выберите сетевой адаптер для инспекции",
        Language::TR => "İncelemek için bir ağ adaptörü seçiniz",
        Language::UK => "Вибрати мережевий адаптер для інспекції",
        Language::ZH => "选择需要监控的网络适配器",
    })
}

pub fn application_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Application protocol",
        Language::DE => "Anwendungs-Protokoll",
        Language::ES => "Protocolo de aplicación",
        Language::FA => "پیوندنامهٔ درخواست",
        Language::FR => "Protocole applicatif",
        Language::IT => "Protocollo applicativo",
        Language::KO => "어플리케이션 프로토콜",
        Language::PL => "Protokół aplikacji",
        Language::PT => "Protocolo de aplicação",
        Language::RO => "Protocol aplicație",
        Language::RU => "Прикладной протокол",
        Language::TR => "Uygulama protokolü",
        Language::UK => "Протокол аплікації",
        Language::ZH => "目标应用层协议",
    }
}

pub fn select_filters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::DE => "Wähle die Filter, die auf den Netzwerkverkehr angewendet werden sollen",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
        Language::FA => "صافی ها را جهت اعمال بر آمد و شد شبکه انتخاب کنید",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::KO => "네트워크 트레픽에 적용할 필터 선택",
        Language::PL => "Wybierz filtry, które mają być zastosowane na ruchu sieciowym",
        Language::PT => "Selecione os filtros a serem aplicados no tráfego de rede",
        Language::RO => "Selectați filtre pentru traficul de rețea",
        Language::RU => "Выберите фильтры для применения к сетевому трафику",
        Language::TR => "Ağ trafiğine uygulanacak filtreleri seçiniz",
        Language::UK => "Вибрати фільтри, які мають бути застосовані до мережевого трафіку",
        Language::ZH => "选择需要监控的目标",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::KO | Language::RO => "Start!",
        Language::ES => "¡Empieza!",
        Language::FA => "شروع!",
        Language::FR => "Commencer!",
        Language::IT => "Avvia!",
        Language::PL => "Rozpocznij!",
        Language::PT => "Começar!",
        Language::RU => "Начать!",
        Language::TR => "Başla!",
        Language::UK => "Почати!",
        Language::ZH => "开始!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddress:",
        Language::DE | Language::FR => "\nAdresse:",
        Language::ES => "\nDirección:",
        Language::FA => "\nنشانی:",
        Language::IT => "\nIndirizzo:",
        Language::KO => "\n주소:",
        Language::PL | Language::TR => "\nAdres:",
        Language::PT => "\nEndereço:",
        Language::RO => "\nAdresă:",
        Language::RU => "\nАдрес:",
        Language::UK => "\nАдреса:",
        Language::ZH => "\n网络地址:",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddresses:",
        Language::DE => "\nAdressen:",
        Language::ES => "\nDirecciones:",
        Language::FA => "\nنشانی ها:",
        Language::FR => "\nAdresses:",
        Language::IT => "\nIndirizzi:",
        Language::KO => "\n주소:",
        Language::PL => "\nAdresy:",
        Language::PT => "\nEndereços:",
        Language::RO => "\nAdrese:",
        Language::RU => "\nАдреса:",
        Language::TR => "\nAdresler:",
        Language::UK => "\nАдреси:",
        Language::ZH => "\n网络地址:",
    }
}

pub fn ip_version_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "IP version",
        Language::DE => "IP Version",
        Language::ES => "Versión IP",
        Language::FA => "نسخهٔ IP",
        Language::FR => "Version IP",
        Language::IT => "Versione IP",
        Language::KO => "IP 버전",
        Language::PL => "Wersja IP",
        Language::PT => "Versão de IP",
        Language::RO => "Versiune IP",
        Language::RU => "Версия IP",
        Language::TR => "IP versiyonu",
        Language::UK => "Версія IP",
        Language::ZH => "目标IP协议版本",
    })
}

pub fn transport_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transport protocol",
        Language::DE => "Netzwerkprotokoll",
        Language::ES | Language::PT => "Protocolo de transporte",
        Language::FA => "پیوندنامهٔ ترابرد",
        Language::FR => "Protocole de transport",
        Language::IT => "Protocollo di trasporto",
        Language::KO => "전송 프로토콜",
        Language::PL => "Protokół transportowy",
        Language::RO => "Protocol de transport",
        Language::RU => "Транспортный протокол",
        Language::TR => "İletişim protokolü",
        Language::UK => "Транспортний протокол",
        Language::ZH => "目标传输协议",
    }
}

pub fn traffic_rate_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Traffic rate:",
        Language::DE => "Daten Frequenz:",
        Language::ES => "Tasa de tráfico:",
        Language::FA => "نرخ آمد و شد:",
        Language::FR => "Fréquence du traffic:",
        Language::IT => "Intensità del traffico:",
        Language::KO => "트레픽 속도:",
        Language::PL => "Prędkość ruchu:",
        Language::PT => "Taxa de tráfego:",
        Language::RO => "Rata de trafic:",
        Language::RU => "Cкорость трафика:",
        Language::TR => "Trafik oranı:",
        Language::UK => "Швидкість руху:",
        Language::ZH => "网络速率图:",
    })
}

pub fn relevant_connections_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Relevant connections:",
        Language::DE => "Relevante Verbindungen:",
        Language::ES => "Conexiones Relevantes:",
        Language::FA => "پیوند های خویشاوند:",
        Language::FR => "Connexions pertinentes:",
        Language::IT => "Connessioni rilevanti:",
        Language::KO => "관련 연결:",
        Language::PL => "Istotne połączenia:",
        Language::PT => "Conexões relevantes:",
        Language::RO => "Conexiuni relevante:",
        Language::RU => "Важные подключения:",
        Language::TR => "İlgili bağlantılar:",
        Language::UK => "Важливі підключення:",
        Language::ZH => "连接详情:",
    })
}

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::DE => "Einstellungen",
        Language::ES => "Ajustes",
        Language::FA => "پیکربندی",
        Language::FR => "Paramètres",
        Language::IT => "Impostazioni",
        Language::KO => "설정",
        Language::PL => "Ustawienia",
        Language::PT => "Configurações",
        Language::RO => "Setări",
        Language::RU => "Настройки",
        Language::TR => "Ayarlar",
        Language::UK => "Налаштування",
        Language::ZH => "设置",
    }
}

pub fn yes_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::DE => "Ja",
        Language::ES => "Sí",
        Language::FA => "بله",
        Language::FR => "Oui",
        Language::IT => "Sì",
        Language::KO => "네",
        Language::PL => "Tak",
        Language::PT => "Sim",
        Language::RO => "Da",
        Language::RU => "Да",
        Language::TR => "Evet",
        Language::UK => "Так",
        Language::ZH => "是",
    })
}

pub fn ask_quit_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::DE => "Bist du sicher, dass du diese Analyse beenden willst?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
        Language::FA => "آیا مطمئن هستید می خواهید از این تحلیل خارج شوید؟",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::KO => "정말로 분석을 종료하겠습니까?",
        Language::PL => "Czy na pewno chcesz zakończyć analizę?",
        Language::PT => "Tem a certeza que deseja sair desta análise?",
        Language::RO => "Sunteți sigur că doriți să renunțați la această analiză?",
        Language::RU => "Вы уверены, что хотите выйти из текущего анализа?",
        Language::TR => "Bu analizden çıkmak istediğine emin misin?",
        Language::UK => "Чи справді хочеш закінчити аналіз?",
        Language::ZH => "您确定退出当前监控吗?",
    })
}

pub fn quit_analysis_translation(language: Language) -> String {
    match language {
        Language::EN => "Quit analysis".to_string(),
        Language::DE => "Analyse beenden".to_string(),
        Language::ES => "Quitar el análisis".to_string(),
        Language::FA => "خروج از تحلیل".to_string(),
        Language::FR => "Quitter l'analyse".to_string(),
        Language::IT => "Interrompi analisi".to_string(),
        Language::KO => "분석종료".to_string(),
        Language::PL => "Zakończ analize".to_string(),
        Language::PT => "Sair da análise".to_string(),
        Language::RO => "Renunță la analiză".to_string(),
        Language::RU => "Закончить анализ".to_string(),
        Language::TR => "Analizden çık".to_string(),
        Language::UK => "Закінчити аналіз".to_string(),
        Language::ZH => "退出监控".to_string(),
    }
}

pub fn ask_clear_all_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::DE => "Bist du sicher, dass du alle Benachrichtigungen löschen willst?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
        Language::FA => "آیا مطمئن هستید می خواهید اعلان ها را پاک کنید؟",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::KO => "알림을 삭제하시겠습니까?",
        Language::PL => "Czy na pewno chcesz wyczyścić powiadomienia?",
        Language::PT => "Tem a certeza que deseja eliminar as notificações?",
        Language::RO => "Sigur doriți să ștergeți notificările?",
        Language::RU => "Вы уверены, что хотите удлить все уведомления?",
        Language::TR => "Bildirimleri temizlemek istediğine emin misin?",
        Language::UK => "Чи справді хочеш видалити всі повідомлення?",
        Language::ZH => "确定清除所有通知?",
    })
}

pub fn clear_all_translation(language: Language) -> String {
    match language {
        Language::EN => "Clear all".to_string(),
        Language::DE => "Alle leeren".to_string(),
        Language::ES => "Borrar todo".to_string(),
        Language::FA => "پاک کردن همه".to_string(),
        Language::FR => "Tout effacer".to_string(),
        Language::IT => "Elimina tutte".to_string(),
        Language::KO => "모두 지우기".to_string(),
        Language::PL => "Wyczyść wszystko".to_string(),
        Language::PT => "Limpar tudo".to_string(),
        Language::RO => "Ștergeți tot".to_string(),
        Language::RU => "Очистить всё".to_string(),
        Language::TR => "Hepsini temizle".to_string(),
        Language::UK => "Видалити все".to_string(),
        Language::ZH => "清除所有".to_string(),
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::DE => "Verstecken",
        Language::ES => "Ocultar",
        Language::FA => "پنهان کردن",
        Language::FR => "Masquer",
        Language::IT => "Nascondi",
        Language::KO => "숨기기",
        Language::PL => "Ukryj",
        Language::PT => "Esconder",
        Language::RO => "Ascundeți",
        Language::RU => "Скрыть",
        Language::TR => "Gizle",
        Language::UK => "Заховати",
        Language::ZH => "隐藏",
    }
}

pub fn no_addresses_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."),
        Language::IT => format!("Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."),
        Language::FR => format!("Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."),
        Language::ES => format!("No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."),
        Language::PL => format!("Nie można zaobserwować żadnego ruchu, ponieważ wybrany adapter nie ma aktywnych adresów...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Jeśli jesteś pewien, że jesteś podłączony do internetu, spróbuj wybrać inny adapter."),
        Language::DE => format!("Es kann kein Netzwerkverkehr beobachtet werden, weil der Adapter keine aktiven Adressen hat...\n\n\
                                 Netzwerkadapter: {adapter}\n\n\
                                 Wenn du dir sicher bist, dass du mit dem Internet verbunden bist, probier einen anderen Adapter auszuwählen."),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку тому що вибраний адаптер немає активних адрес... \n\n\
                                 Мережквий адаптер: {adapter}\n\n\
                                 Якщо Ти впевнений, що підключений до інтернету, спробуй вибрати інший адаптер."),
        Language::ZH => format!("您选择的网络适配器当前无活动网络...\n\n\
                                网络适配器: {adapter}\n\n\
                                如果您确信您已成功连接互联网, 请尝试选择其他网络适配器."),
        Language::RO => format!("Niciun trafic nu poate fi observat deoarece adaptorul selectat nu are adrese active...\n\n\
                                Adaptor de rețea: {adapter}\n\n\
                                Dacă sunteți sigur că sunteți conectat la internet, încercați să alegeți un alt adaptor."),
        Language::KO => format!("선택한 어댑터에 유효한 주소가 없기 때문에 트래픽을 확인할 수 없습니다...\n\n\
                                네트워크 어뎁터: {adapter}\n\n\
                                인터넷이 연결되어있다면 다른 어댑터로 시도해보세요."),
        Language::TR => format!("Seçtiğiniz adaptör aktif bir adrese sahip olmadığı için hiç bir trafik izlenemez...\n\n\
                                 Ağ adaptörü: {adapter}\n\n\
                                 Eğer gerçekten internete bağlı olduğunuza eminseniz, başka bir adaptör seçmeyi deneyiniz."),
        Language::RU => format!("Наблюдение за трафиком не возможно, потому что Вы выбрали интерфейс без активного адреса...\n\n\
                                 Сетевой интерфейс: {adapter}\n\n\
                                 Если Вы уверены, что подключены к Интернету, попробуйте выбрать другой интерфейс."),
        Language::PT => format!("Não é possível observar tráfego porque o adaptador que selecionou não tem endereços ativos...\n\n\
                                Adaptador de rede: {adapter}\n\n\
                                Se tiver a certeza que está ligado à internet, tente escolher um adaptador diferente."),
        Language::FA => format!("هیچ آمد و شدی قابل مشاهده نیست چون مبدلی که انتخاب کرده اید هیچ نشانی فعالی ندارد...\n\n\
                                مبدل شبکه: {adapter}\n\n\
                                اگر مطمئن هستید به اینترنت وصل هستید، سعی کنید مبدل متفاوتی را انتخاب کنید."),
    })
}

pub fn waiting_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"),
        Language::IT => format!("Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"),
        Language::FR => format!("Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"),
        Language::ES => format!("Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"),
        Language::PL => format!("Nie zaobserowano żadnego ruchu sieciowego. Oczekiwanie na pakiety...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Czy na pewno jesteś podłączony do internetu i wybrałeś właściwy adapter?"),
        Language::DE => format!("Noch kein Netzwerkverkehr beobachtet. Warte auf Pakete...\n\n\
                                 Netzwerkadapter: {adapter}\n\n\
                                 Bist du sicher, dass du mit dem Internet verbunden bist und den richtigen Adapter ausgewählt hast?"),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку. Очікування на пакети...\n\n\
                                 Мережквий адаптер: {adapter}\n\n\
                                 Чи Ти дійсно підключений до інтернету і вибрав відповідний мережевий адаптер?"),
        Language::ZH => format!("暂无流量数据. 等待网络活动中......\n\n\
                                 网络适配器: {adapter}\n\n\
                                 您确信您已成功连接到互联网, 并选择了当前正在使用的的网络适配器吗?"),
        Language::RO => format!("Nu a fost observat încă trafic. Se așteaptă pachetele de rețea...\n\n\
                                Adaptor de rețea: {adapter}\n\n\
                                Ești sigur că ești conectat la internet și ai selectat adaptorul corect?"),
        Language::KO => format!("아직 트래픽이 관찰되지 않았습니다. 네트워크 패킷 대기 중...\n\n\
                                네트워크 어뎁터: {adapter}\n\n\
                                인터넷에 연결되어 있고 올바른 어댑터를 선택하셨습니까?"),
        Language::TR => format!("Henüz bir trafik algılanamadı. Ağ paketleri için bekleniyor...\n\n\
                                 Ağ adaptörü: {adapter}\n\n\
                                 İnternete bağlı olduğunuza ve doğru adaptörü seçtiğinize emin misiniz?"),
        Language::RU => format!("Трафик не обнаружен. Ожидаем сетевые пакеты...\n\n\
                                 Сетевой интерфейс: {adapter}\n\n\
                                 Вы уверены, что подключены к Интернету и выбрали правильный интерфейс?"),
        Language::PT => format!("Ainda não foi observado tráfego. Aguardando por pacotes...\n\n\
                                Adaptador de rede: {adapter}\n\n\
                                Tem a certeza de que está ligado à internet e selecionou o adaptador correto?"),
        Language::FA => format!("هنوز هیچ آمد و شدی مشاهده نشده است. در حال انتظار برای بسته های شبکه...\n\n
                                مبدل شبکه: {adapter}\n\n
                                آیا مطمئن هستید به اینترنت وصل هستید و مبدل درست را انتخاب کرده اید؟"),
    })
}

pub fn some_observed_translation(
    language: Language,
    observed: &str,
    filters: &str,
) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => format!("Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{filters}"),
        Language::IT => format!("Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati...\n\n{filters}"),
        Language::FR => format!("Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés...\n\n{filters}"),
        Language::ES => format!("Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados...\n\n{filters}"),
        Language::PL => format!("Suma przechwyconych pakietów: {observed}\n\n\
                                 Przefiltrowane pakiety: 0\n\n\
                                 Niektóre pakiety zostały przechwycone, ale żaden nie został wybrany zgodnie z wskazanymi filtrami...\n\n{filters}"),
        Language::DE => format!("Anzahl der empfangenen Pakete: {observed}\n\n\
                                 gefilterte Pakete: 0\n\n\
                                 Ein Paar Pakete wurden empfangen, aber es entsprechen noch keine den spezifizierten Filtern...\n\n{filters}"),
        Language::UK => format!("Сума перехоплених пакетів: {observed}\n\n\
                                 Відфільтровані пакеті: 0\n\n\
                                 Деякі пакети були перехоплені, але жоден з них не був вибраний відповідно до вказаних фільтрів...\n\n{filters}"),
        Language::ZH => format!("监测到的数据包总数: {observed}\n\n\
                                 目标数据包总数: 0\n\n\
                                 当前已监测到一些数据包, 但其中并未包含您的目标数据包......\n\n{filters}"),
        Language::RO => format!("Total pachete interceptate: {observed}\n\n\
                                Pachete filtrate: 0\n\n\
                                Unele pachete au fost interceptate, dar încă niciunul nu a fost selectat conform filtrelor pe care le-ați specificat...\n\n{filters}"),
        Language::KO => format!("감지한 총 패킷: {observed}\n\n\
                                필터링된 패킷: 0\n\n\
                                일부 패킷이 감지되었지만, 지정한 필터에 따라 선택되지 않았습니다...\n\n{filters}"),
        Language::TR => format!("Toplam yakalanan paketler: {observed}\n\n\
                                 Filterelenen paketler: 0\n\n\
                                 Bazı paketler yakalandı, fakat belirttiğiniz filtrelere göre hiç biri seçilmedi...\n\n{filters}"),
        Language::RU => format!("Всего пакетов перехвачено: {observed}\n\n\
                                 Фильтровано пакетов: 0\n\n\
                                 Сетевые пакеты были перехвачены, но ни один из них не соответствует заданным фильтрам...\n\n{filters}"),
        Language::PT => format!("Total de pacotes interceptados: {observed}\n\n\
                                Pacotes filtrados: 0\n\n\
                                Alguns pacotes foram interceptados, mas nenhum deles foi selecionado de acordo com os filtros especificados...\n\n{filters}"),
        Language::FA => format!("مجموع بسته های رهگیری شده: {observed}\n\n\
                                بسته های صاف شده: 0\n\n\
                                شماری از بسته ها رهگیری شده اند، ولی هنوز هیچ کدام بر اساس صافی تعیین شده شما انتخاب نشده اند...\n\n{filters}"),
    })
}

pub fn filtered_packets_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered packets:\n   {filtered} ({percentage} of the total)"),
        Language::DE => format!("Gefilterte Pakete:\n   {filtered} ({percentage} der Gesamtzahl)"),
        Language::ES => format!("Paquetes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::FA => format!("بسته های صاف شده\n   {filtered} ({percentage} از مجموع)"),
        Language::FR => format!("Paquets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::IT => format!("Pacchetti filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::KO => format!("필터링된 패킷:\n   {filtered} ({percentage} 의 일부)"),
        Language::PL => format!("Przefiltrowane pakiety:\n   {filtered} ({percentage} z całości)"),
        Language::PT => format!("Pacotes filtrados:\n   {filtered} ({percentage} do total)"),
        Language::RO => format!("Pachete filtrate:\n   {filtered} ({percentage} din total)"),
        Language::RU => format!("Отфильтровано пакетов:\n   {filtered} ({percentage}% от общего числа)"),
        Language::TR => format!("Filtrelenen paketler:\n   {filtered} toplamın ({percentage})"),
        Language::UK => format!("Відфільтровані пакети:\n   {filtered} ({percentage} від загальної суми)"),
        Language::ZH => format!("目标数据包计数:\n   {filtered} (占所有数据包的 {percentage})"),
    })
}

pub fn filtered_bytes_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered bytes:\n   {filtered} ({percentage} of the total)"),
        Language::DE => format!("Gefilterte Bytes:\n   {filtered} ({percentage} der Gesamtzahl)"),
        Language::ES => format!("Bytes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::FA => format!("بایت های صاف شده\n   {filtered} ({percentage} از مجموع)"),
        Language::FR => format!("Octets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::IT => format!("Byte filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::KO => format!("필터링된 바이트:\n   {filtered} ({percentage} 의 일부)"),
        Language::PL => format!("Przechwycone bajty:\n   {filtered} ({percentage} całości)"),
        Language::PT => format!("Bytes filtrados:\n   {filtered} ({percentage} do total)"),
        Language::RO => format!("Octeți filtrați:\n   {filtered} ({percentage} din total)"),
        Language::RU => format!("Отфильтровано байт:\n   {filtered} ({percentage}% от общего числа)"),
        Language::TR => format!("Filtrelenen bayt:\n   {filtered} toplamın ({percentage})"),
        Language::UK => format!("Відфільтровані байти:\n   {filtered} ({percentage} від загальної суми)"),
        Language::ZH => format!("目标网络流量计数:\n   {filtered} (占所有网络流量的 {percentage})"),
    })
}

pub fn filtered_application_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Filtered packets per application protocol:",
        Language::DE => "Gefilterte Pakete je Anwendungs-Protokoll:",
        Language::ES => "Paquetes filtrados por protocolo de aplicación:",
        Language::FA => "بسته های صاف شده برای هر پیوندنامهٔ درخواست:",
        Language::FR => "Paquets filtrés par protocole applicatif:",
        Language::IT => "Pacchetti filtrati per protocollo applicativo:",
        Language::KO => "애플리케이션 프로토콜당 필터링된 패킷 수:",
        Language::PL => "Przefiltrowane pakiety według protokołu aplikacji:",
        Language::PT => "Pacotes filtrados por protocolo de aplicação:",
        Language::RO => "Pachete filtrate pe protocol de aplicație:",
        Language::RU => "Отфильтровано пакетов прикладного протокола:",
        Language::TR => "Uygulama protokolü bazında filtrelenen paketler:",
        Language::UK => "Відфільтровані пакети протоколу аплікації/програми:",
        Language::ZH => "按应用层协议分类的目标数据包计数:",
    })
}

pub fn no_favorites_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => "Nothing to show at the moment.\n\
                         To add a connection to your favorites, click on the star symbol near the connection.",
        Language::IT => "Nulla da vedere per il momento.\n\
                         Per aggiungere una connessione ai tuoi preferiti, clicca sul simbolo della stella vicino alla connessione.",
        Language::FR => "Rien a voir pour le moment.\n\
                         Pour ajouter une connexion à vos favoris, cliquez sur l'étoile à côté de la connexion.",
        Language::ES => "Nada que mostrar por el momento.\n\
                         Para añadir una conexión a sus favoritos, haga clic en el símbolo de la estrella situado junto a la conexión.",
        Language::PL => "Nie ma nic do pokazania w tej chwili.\n\
                         Aby dodać połączenie do ulubionych, kliknij na ikonę 'gwiazdki' obok połączenia.",
        Language::DE => "Im Moment nichts zu zeigen.\n\
                         Um eine Verbindung zu deinen Favoriten hinzuzufügen, klick das auf das Stern-Symbol neben der Verbindung.",
        Language::UK => "Немає, що показати в цей момент.\n\
                         Щоб додати підключення до улюблених, натисни на іконку 'зірочки' біля підключення.",
        Language::ZH => "收藏夹还是空的.\n\
                         小贴士: 点击连接右侧的小星星即可收藏到这里哦.",
        Language::RO => "Nimic de arătat în acest moment.\n\
                        Pentru a adăuga o conexiune la favorite, faceți clic pe simbolul stea din apropierea conexiunii.",
        Language::KO => "현재는 보여줄게 없습니다.\n\
                         즐겨찾기에 연결을 추가하려면 별을 눌러주세요.",
        Language::TR => "Şu an gösterecek bir şey yok.\n\
                         Favorilere bağlantı eklemek için, bağlantı yanındaki yıldız sembolüne tıklayınız.",
        Language::RU => "Нечего показать в настоящий момент.\n\
                         Для добавления соединения в избранные, нажмите на символ звезды возле соединения.",
        Language::PT => "Nada para mostrar de momento.\n\
                         Para adicionar uma conexão aos seus favoritos, clique na estrela perto da conexão.",
        Language::FA => "در حال حاضر هیچ چیزی برای نمایش نیست.\n\
                        برای افزودن یک پیوند به پسندیده های خود، روی نشان ستاره کنار پیوند کلیک کنید.",
    })
}

pub fn error_translation(language: Language, error: &str) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
        Language::PL => format!(
            "Wystąpił błąd! \n\n\
                                {error}"
        ),
        Language::DE => format!(
            "Es ist ein Fehler aufgetreten! \n\n\
                                {error}"
        ),
        Language::UK => format!(
            "Виступила помилка! \n\n\
                                {error}"
        ),
        Language::ZH => format!(
            "发生了一些错误! \n\n\
                                {error}"
        ),
        Language::RO => format!(
            "A apărut o eroare! \n\n\
                                {error}"
        ),
        Language::KO => format!(
            "오류가 발생하였습니다! \n\n\
                                {error}"
        ),
        Language::TR => format!(
            "Bir hata oluştu! \n\n\
                                 {error}"
        ),
        Language::RU => format!(
            "Произошла ошибка! \n\n\
                                 {error}"
        ),
        Language::PT => format!(
            "Ocorreu um erro! \n\n\
                                {error}"
        ),
        Language::FA => format!(
            "خطایی رخ داد! \n\n\
                                {error}"
        ),
    })
}

pub fn both_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "both",
        Language::DE => "beide",
        Language::ES | Language::PT => "ambos",
        Language::FA => "هر دو",
        Language::FR => "les deux",
        Language::IT => "entrambi",
        Language::KO => "둘다",
        Language::PL => "oba",
        Language::RO => "ambele",
        Language::RU => "оба",
        Language::TR => "ikiside",
        Language::UK => "обидва",
        Language::ZH => "皆需",
    }
}

// pub fn all_protocols_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All protocols",
//         Language::DE => "Alle Protokolle",
//         Language::ES => "Todos los protocolos",
//         Language::FA => "همهٔ پیوندنامه ها",
//         Language::FR => "Tous les protocoles",
//         Language::IT => "Tutti i protocolli",
//         Language::PL => "Wszystkie protokoły",
//         Language::RU => "Все протоколы",
//     }
// }

pub fn all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "All",
        Language::DE => "Alle",
        Language::ES | Language::PT => "Todos",
        Language::FA => "همه",
        Language::FR => "Tous",
        Language::IT => "Tutti",
        Language::KO => "모두",
        Language::PL => "Wszystkie",
        Language::RO => "Toate",
        Language::RU => "Всё",
        Language::TR => "Hepsi",
        Language::UK => "Усі",
        Language::ZH => "所有",
    }
}

pub fn packets_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets per second",
        Language::DE => "pakete pro Sekunde",
        Language::ES => "paquetes por segundo",
        Language::FA => "بسته در ثانیه",
        Language::FR => "paquets par seconde",
        Language::IT => "pacchetti al secondo",
        Language::KO => "초당 패킷",
        Language::PL => "pakiety na sekundę",
        Language::PT => "pacotes por segundo",
        Language::RO => "pachete pe secundă",
        Language::RU => "пакектов в секунду",
        Language::TR => "saniye başı paket",
        Language::UK => "пакети на секунду",
        Language::ZH => "数据包",
    }
}

pub fn bytes_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "bytes per second",
        Language::DE => "bytes pro Sekunde",
        Language::ES | Language::PT => "bytes por segundo",
        Language::FA => "بایت در ثانیه",
        Language::FR => "octets par seconde",
        Language::IT => "byte al secondo",
        Language::KO => "초당 바이트",
        Language::PL => "bajty na sekundę",
        Language::RO => "octeți pe secundă",
        Language::RU => "байтов в секунду",
        Language::TR => "saniye başı bayt",
        Language::UK => "байти на секунду",
        Language::ZH => "网络流量",
    }
}

pub fn recent_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most recent",
        Language::DE => "zuletzt",
        Language::ES => "más reciente",
        Language::FA => "آخرین",
        Language::FR => "la plus récente",
        Language::IT => "più recenti",
        Language::KO => "가장 최근",
        Language::PL => "najnowsze",
        Language::PT => "mais recente",
        Language::RO => "cea mai recentă",
        Language::RU => "новейшие",
        Language::TR => "en son",
        Language::UK => "найновіші",
        Language::ZH => "按时间",
    }
}

pub fn packets_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most packets",
        Language::DE => "meiste Pakete",
        Language::ES => "mayoría de los paquetes",
        Language::FA => "بیشترین بسته ها",
        Language::FR => "le plus de paquets",
        Language::IT => "più pacchetti",
        Language::KO => "대부분의 패킷",
        Language::PL => "najwięcej pakietów",
        Language::PT => "mais pacotes",
        Language::RO => "cele mai multe pachete",
        Language::RU => "больше всего пакетов",
        Language::TR => "en çok paket",
        Language::UK => "найбільше пакетів",
        Language::ZH => "按数据包",
    }
}

pub fn bytes_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most bytes",
        Language::DE => "meiste Bytes",
        Language::ES => "mayoría de los bytes",
        Language::FA => "بیشترین بایت ها",
        Language::FR => "le plus de données",
        Language::IT => "più byte",
        Language::KO => "대부분의 바이트",
        Language::PL => "najwięcej bajtów",
        Language::PT => "mais bytes",
        Language::RO => "cei mai mulți octeți",
        Language::RU => "больше всего байт",
        Language::TR => "en çok bayt",
        Language::UK => "найбільше байтів",
        Language::ZH => "按流量",
    }
}

pub fn favorite_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "favorites",
        Language::DE => "Favoriten",
        Language::ES | Language::PT => "favoritos",
        Language::FA => "پسندیده ها",
        Language::FR => "favoris",
        Language::IT => "preferiti",
        Language::KO => "즐겨찾기",
        Language::PL => "ulubione",
        Language::RO => "favorite",
        Language::RU => "избранное",
        Language::TR => "favoriler",
        Language::UK => "улюблені",
        Language::ZH => "收藏夹",
    }
}

pub fn notifications_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::DE => "Personalisier deine Benachrichtigungen",
        Language::ES => "Personaliza tus notificaciones",
        Language::FA => "اعلان های خود را سفارشی کنید",
        Language::FR => "Personnalisez vos notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::KO => "사용자 지정 알림",
        Language::PL => "Dostosuj powiadomienia",
        Language::PT => "Personalize as suas notificações",
        Language::RO => "Personalizați-vă notificările",
        Language::RU => "Настройка уведомлений",
        Language::TR => "Bildirimlerinizi özelleştirin",
        Language::UK => "Достосуй повідомлення",
        Language::ZH => "自定义通知",
    })
}

pub fn appearance_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::DE => "Wähl dein Lieblingsdesign",
        Language::ES => "Elige tu tema favorito",
        Language::FA => "زمینه دلخواه خود را انتخاب کنید",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::IT => "Scegli il tuo tema preferito",
        Language::KO => "태마를 선택하세요",
        Language::PL => "Wybierz swój ulubiony motyw",
        Language::PT => "Escolha o seu tema favorito",
        Language::RO => "Selectați tema preferată",
        Language::RU => "Выберите предпочительную тему",
        Language::TR => "Favori temanızı seçin",
        Language::UK => "Вибери улюблену тему",
        Language::ZH => "选择您喜欢的主题",
    })
}

pub fn languages_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select your language",
        Language::DE => "Stell deine Sprache ein",
        Language::ES => "Selecciona tu idioma",
        Language::FA => "زبان خود را انتخاب کنید",
        Language::FR => "Sélectionnez votre langue",
        Language::IT => "Seleziona la lingua",
        Language::KO => "언어를 선택하세요",
        Language::PL => "Wybierz język",
        Language::PT => "Selecione o seu idioma",
        Language::RO => "Selectați limba",
        Language::RU => "Выберите язык",
        Language::TR => "Dilinizi seçin",
        Language::UK => "Вибери мову",
        Language::ZH => "选择显示语言",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters:",
        Language::DE => "Aktive Filter:",
        Language::ES => "Filtros activos:",
        Language::FA => "صافی های فعال:",
        Language::FR => "Filtres actifs",
        Language::IT => "Filtri attivi:",
        Language::KO => "활성화된 필터:",
        Language::PL => "Aktywne filtry:",
        Language::PT => "Filtros ativos:",
        Language::RO => "Filtre active:",
        Language::RU => "Выбранные фильтры:",
        Language::TR => "Aktif filtreler:",
        Language::UK => "Активні фільтри:",
        Language::ZH => "活动的过滤器:",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::DE => "keine",
        Language::ES => "ninguno",
        Language::FA => "هیچ کدام",
        Language::FR => "aucun",
        Language::IT => "nessuno",
        Language::KO => "없음",
        Language::PL => "brak",
        Language::PT => "nenhum",
        Language::RO => "niciunul",
        Language::RU => "не выбран",
        Language::TR => "hiç biri",
        Language::UK => "бракує",
        Language::ZH => "无",
    }
}

pub fn yeti_night_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original dark theme",
        Language::DE => "Sniffnets urspüngliches, dunkles Design",
        Language::ES => "Tema oscuro original de Sniffnet",
        Language::FA => "زمینه تاریک اصلی Sniffnet",
        Language::FR => "Thème original sombre de Sniffnet",
        Language::IT => "Il tema scuro originale di Sniffnet",
        Language::KO => "Sniffnet의 기본 다크테마",
        Language::PL => "Oryginalny, ciemny motyw Sniffnet",
        Language::PT => "Tema escuro original de Sniffnet",
        Language::RO => "Tema întunecată originală Sniffnet",
        Language::RU => "Оригинальная тёмная тема Sniffnet'а",
        Language::TR => "Sniffnet'in orjinal koyu teması",
        Language::UK => "Оригінальний, темний мотив Sniffnet",
        Language::ZH => "Sniffnet暗黑",
    }
}

pub fn yeti_day_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original light theme",
        Language::DE => "Sniffnets urspüngliches, helles Design",
        Language::ES | Language::PT => "Tema claro original de Sniffnet",
        Language::FA => "زمینه روشن اصلی Sniffnet",
        Language::FR => "Thème original clair de Sniffnet",
        Language::IT => "Il tema chiaro originale di Sniffnet",
        Language::KO => "Sniffnet의 기본 라이트테마",
        Language::PL => "Oryginalny, jasny motyw Sniffnet",
        Language::RO => "Tema deschisă originală Sniffnet",
        Language::RU => "Оригинальная светая тема Sniffnet'а",
        Language::TR => "Sniffnet'in orjinal açık teması",
        Language::UK => "Оригінальний, світлий мотив Sniffnet",
        Language::ZH => "Sniffnet浅色",
    }
}

pub fn deep_sea_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "To dive into network traffic",
        Language::DE => "Um in den Netzwerkverkehr einzutauchen",
        Language::ES => "Para sumergirse en el tráfico de la red",
        Language::FA => "شیرجه رفتن در آمد و شد شبکه",
        Language::FR => "Pour plonger dans votre trafic réseau",
        Language::IT => "Per immergersi nel traffico di rete",
        Language::KO => "네트워크 트레픽으로 바로가기",
        Language::PL => "Aby zanurzyć się w ruchu sieciowym",
        Language::PT => "Para mergulhar no tráfego de rede",
        Language::RO => "Pentru a vă scufunda în traficul de rețea",
        Language::RU => "Для погружения в сетевой трафик",
        Language::TR => "Ağ trafiğine dalmak",
        Language::UK => "Проаналізувати мережевий рух",
        Language::ZH => "潜入网络活动的海洋",
    }
}

pub fn mon_amour_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Lovely theme made for dreamers",
        Language::DE => "Liebevolles Design für Träumer",
        Language::ES => "Tema encantador hecho para soñadores",
        Language::FA => "زمینه دلپذیر ساخته شده برای رویا پردازان",
        Language::FR => "Thème romantique fait pour les rêveurs",
        Language::IT => "Tema incantevole fatto per i sognatori",
        Language::KO => "사랑스러운 몽환가들을 위한 테마",
        Language::PL => "Uroczy motyw stworzony dla marzycieli",
        Language::PT => "Tema encantador feito para sonhadores",
        Language::RO => "O temă minunată creată pentru visători",
        Language::RU => "Милая тема для мечтателей",
        Language::TR => "Hayal perestler için yapılmış güzel tema",
        Language::UK => "Прекрасна тема для мрійників",
        Language::ZH => "梦想家的主题",
    }
}

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::DE => "Ankommend",
        Language::ES => "Entrante",
        Language::FA => "ورودی",
        Language::FR => "Entrant",
        Language::IT => "In entrata",
        Language::KO => "수신중",
        Language::PL => "Przychodzące",
        Language::PT => "Entrando",
        Language::RO => "de intrare",
        Language::RU => "Входящий",
        Language::TR => "Gelen",
        Language::UK => "Вхідні",
        Language::ZH => "入站",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::DE => "Ausgehend",
        Language::ES => "Saliente",
        Language::FA => "خروجی",
        Language::FR => "Sortant",
        Language::IT => "In uscita",
        Language::KO => "발신중",
        Language::PL => "Wychodzące",
        Language::PT => "Saindo",
        Language::RO => "de ieșire",
        Language::RU => "Исходящий",
        Language::TR => "Giden",
        Language::UK => "Вихідні",
        Language::ZH => "出站",
    }
}

pub fn notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Notifications",
        Language::DE => "Benachrichtigungen",
        Language::ES => "Notificaciones",
        Language::FA => "اعلان ها",
        Language::IT => "Notifiche",
        Language::KO => "알림",
        Language::PL => "Powiadomienia",
        Language::PT => "Notificações",
        Language::RO => "Notificări",
        Language::RU => "Уведомления",
        Language::TR => "Bildirimler",
        Language::UK => "Повідомлення",
        Language::ZH => "通知",
    }
}

pub fn style_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Style",
        Language::DE | Language::RO | Language::TR => "Stil",
        Language::ES | Language::PT => "Estilo",
        Language::FA => "شیوه",
        Language::IT => "Stile",
        Language::KO => "스타일",
        Language::PL => "Styl",
        Language::RU | Language::UK => "Стиль",
        Language::ZH => "主题",
    }
}

pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::DE => "Sprache",
        Language::ES => "Idioma",
        Language::FA => "زبان",
        Language::FR => "Langue",
        Language::IT => "Lingua",
        Language::KO => "언어",
        Language::PL => "Język",
        Language::PT => "Língua",
        Language::RO => "Limbă",
        Language::RU => "Язык",
        Language::TR => "Dil",
        Language::UK => "Мова",
        Language::ZH => "语言",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::DE => "Übersicht",
        Language::ES => "Resumen",
        Language::FA => "نمای کلی",
        Language::FR => "Résumé",
        Language::IT => "Panoramica",
        Language::KO => "개요",
        Language::PL => "Przegląd",
        Language::PT => "Visão geral",
        Language::RO => "Prezentare generală",
        Language::RU => "Обзор",
        Language::TR => "Ön izleme",
        Language::UK => "Огляд",
        Language::ZH => "概览",
    }
}

// pub fn inspect_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Inspect",
//         Language::DE => "Überprüfen",
//         Language::ES => "Inspeccionar",
//         Language::FA => "بازرسی",
//         Language::FR => "Inspecter",
//         Language::IT => "Ispeziona",
//         Language::PL => "Sprawdź",
//         Language::RU => "Инспектировать",
//     }
// }

pub fn packets_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a packets threshold is exceeded",
        Language::DE => "Benachrichtige mich, wenn die Pakete eine Schwelle überschreiten",
        Language::ES => "Notificarme cuando se supere un límite de paquetes",
        Language::FA => "به من اطلاع بده وقتی آستانه یک بسته فراتر رفت",
        Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
        Language::IT => "Notificami quando una soglia di pacchetti è superata",
        Language::KO => "패킷 임계값을 초과하면 알림",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg pakietów",
        Language::PT => "Notifique-me quando um limite de pacotes for excedido",
        Language::RO => "Anunță-mă când este depășit un prag de pachete",
        Language::RU => "Уведомить, когда порог по частоте пакетов превышен",
        Language::TR => "Paket eşiği aşıldığında beni bilgilendir",
        Language::UK => "Повідом мене про переліміт пакетів",
        Language::ZH => "超过设定的数据包数量阈值时通知我",
    }
}

pub fn bytes_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a bytes threshold is exceeded",
        Language::DE => "Benachrichtige mich, wenn die Bytes eine Schwelle überschreiten",
        Language::ES => "Notificarme cuando se exceda un límite de bytes",
        Language::FA => "به من اطلاع بده وقتی آستانه یک بایت فراتر رفت",
        Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
        Language::IT => "Notificami quando una soglia di byte è superata",
        Language::KO => "바이트 임계값을 초과하면 알림",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg bajtów",
        Language::PT => "Notifique-me quando um limite de bytes for excedido",
        Language::RO => "Anunță-mă când este depășit un prag de octeți",
        Language::RU => "Уведомить, когда порог по полосе в байтах превышен",
        Language::TR => "Bayt eşiği aşıldığında beni bilgilendir",
        Language::UK => "Повідом мене про переліміт байтів",
        Language::ZH => "超过设定的网络流量阈值时通知我",
    }
}

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::DE => "(pro Sekunde)",
        Language::ES | Language::PT => "(por segundo)",
        Language::FA => "(در ثانیه)",
        Language::FR => "(par seconde)",
        Language::IT => "(al secondo)",
        Language::KO => "(초당)",
        Language::PL => "(na sekundę)",
        Language::RO => "(pe secundă)",
        Language::RU => "(в секунду)",
        Language::TR => "(her saniye)",
        Language::UK => "(на секунду)",
        Language::ZH => "(每秒) ",
    }
}

pub fn specify_multiples_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "; you can also specify 'K', 'M' and 'G'",
        Language::DE => "; du kannst auch 'K', 'M' und 'G' festlegen",
        Language::ES => "; también puede especificar 'K', 'M' y 'G'",
        Language::FA => "؛ شما همچنین می توانید 'M'، 'K' و 'G' را تعیین کنید",
        Language::FR => "; vous pouvez également spécifier 'K', 'M' et 'G'",
        Language::IT => "; puoi anche specificare 'K', 'M' e 'G'",
        Language::KO => "; 지정가능합니다 'K', 'M', 'G'",
        Language::PL => "; możesz również określić 'K', 'M' i 'G'",
        Language::PT => "; também pode especificar 'K', 'M' e 'G'",
        Language::RO => "; puteți specifica 'K', 'M', 'G'",
        Language::RU => "; Так же можно указать 'K', 'M' или 'G'",
        Language::TR => "; şunları da kullanabilirsin 'K', 'M' ve 'G'",
        Language::UK => "; можеш також вибрати 'K', 'M' i 'G'",
        Language::ZH => "您可指定 'K', 'M', 'G'",
    }
}

pub fn favorite_notification_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when new data are exchanged from my favorites",
        Language::DE => "Benachrichtige mich, wenn neue Daten mit meinen Favoriten ausgetauscht werden",
        Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
        Language::FA => "به من اطلاع بده وقتی داده جدید از پسندیده های من مبادله شد",
        Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
        Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
        Language::KO => "즐겨찾기에서 새 데이터가 교환될 때 알림",
        Language::PL => "Powiadom mnie, gdy nowe dane z moich ulubionych zostaną wymienione",
        Language::PT => "Notificar-me quando novos dados forem trocados dos meus favoritos",
        Language::RO => "Anunță-mă când sunt transferate date noi de la favoritele mele",
        Language::RU => "Уведомить, если произошёл обмен данными в соединениях из избранного",
        Language::TR => "Favorilerimde veri akışı olduğunda beni uyar",
        Language::UK => "Повідом мене, коли буде обмін даними з моїх улюблених",
        Language::ZH => "收藏夹内的连接有新活动时通知我",
    }
}

pub fn threshold_translation(language: Language) -> String {
    match language {
        Language::EN => "Threshold: ".to_string(),
        Language::DE => "Schwellenwert: ".to_string(),
        Language::ES => "Límite: ".to_string(),
        Language::FA => "آستانه:".to_string(),
        Language::FR => "Seuil: ".to_string(),
        Language::IT => "Soglia: ".to_string(),
        Language::KO => "임계값: ".to_string(),
        Language::PL => "Próg: ".to_string(),
        Language::PT => "Limite: ".to_string(),
        Language::RO => "Prag: ".to_string(),
        Language::RU => "Порог: ".to_string(),
        Language::TR => "Eşik: ".to_string(),
        Language::UK => "Ліміт: ".to_string(),
        Language::ZH => "阈值: ".to_string(),
    }
}

pub fn volume_translation(language: Language, value: u8) -> String {
    match language {
        Language::EN | Language::FR | Language::IT | Language::PT => format!("Volume: {value:^3}%"),
        Language::DE => format!("Lautstärke: {value:^3}%"),
        Language::ES => format!("Volumen: {value:^3}%"),
        Language::FA => format!("حجم: {value:^3}%"),
        Language::KO => format!("볼륨: {value:^3}%"),
        Language::PL => format!("Głośność: {value:^3}%"),
        Language::RO => format!("Volum: {value:^3}%"),
        Language::RU => format!("Объём: {value:^3}%"),
        Language::TR => format!("Ses: {value:^3}%"),
        Language::UK => format!("Гучність: {value:^3}%"),
        Language::ZH => format!("通知音量: {value:^3}%"),
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound:",
        Language::DE => "Ton:",
        Language::ES => "Sonido:",
        Language::FA => "صدا:",
        Language::FR => "Son:",
        Language::IT => "Suono:",
        Language::KO => "사운드:",
        Language::PL => "Dźwięk:",
        Language::PT => "Som:",
        Language::RO => "Sunet:",
        Language::TR => "Ses:",
        Language::RU | Language::UK => "Звук:",
        Language::ZH => "通知音:",
    }
}

pub fn open_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Open full report",
        Language::DE => "Kompletten Bericht öffnen",
        Language::ES => "Abrir el informe completo",
        Language::FA => "گزارش کامل را باز کن",
        Language::FR => "Ouvrir le rapport complet",
        Language::IT => "Apri report completo",
        Language::KO => "전체 보고서 열기",
        Language::PL => "Otwórz pełny raport",
        Language::PT => "Abrir relatório completo",
        Language::RO => "Deschideți raport complet",
        Language::RU => "Открыть полный отчёт",
        Language::TR => "Tam raporu aç",
        Language::UK => "Відкрий повний рапорт",
        Language::ZH => "打开完整报告",
    }
}

pub fn bytes_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bytes threshold exceeded!",
        Language::DE => "Byte-Schwellenwert überschritten!",
        Language::ES => "¡Límite de bytes superado!",
        Language::FA => "آستانه بایت فراتر رفت!",
        Language::FR => "Seuil de donnée atteint!",
        Language::IT => "Soglia di Byte superata!",
        Language::KO => "바이트 임계값 초과!",
        Language::PL => "Próg bajtów przekroczony!",
        Language::PT => "Limite de bytes excedido!",
        Language::RO => "Prag de octeți depășit!",
        Language::RU => "Порог в байтах превышен!",
        Language::TR => "Bayt eşik değeri aşıldı!",
        Language::UK => "Ліміт байтів перевищено!",
        Language::ZH => "达到设定的网络流量阈值!",
    }
}

pub fn bytes_exceeded_value_translation(language: Language, value: &str) -> String {
    let trimmed_value = value.trim();
    match language {
        Language::EN => format!("{trimmed_value} bytes have been exchanged"),
        Language::DE => format!("{trimmed_value} Bytes wurden ausgetauscht"),
        Language::ES => format!("{trimmed_value} byte/s han sido intercambiado/s"),
        Language::FA => format!("{trimmed_value} بایت مبادله شده است"),
        Language::FR => format!("{trimmed_value} octets ont été échangé"),
        Language::IT => format!("{trimmed_value} byte sono stati scambiati"),
        Language::KO => format!("바이트 {trimmed_value} 가 교환되었습니다"),
        Language::PL => format!("Wymieniono {trimmed_value} bajtów"),
        Language::PT => format!("Foram trocados {trimmed_value} bytes"),
        Language::RO => format!("au fost transferați {trimmed_value} octeți"),
        Language::RU => format!("{trimmed_value} байт обмена информацией"),
        Language::TR => format!("{trimmed_value} bayt aktarıldı"),
        Language::UK => format!("{trimmed_value} байтів було обміняно"),
        Language::ZH => format!("已交换字节 {trimmed_value}"),
    }
}

pub fn packets_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Packets threshold exceeded!",
        Language::DE => "Paket-Schwellenwert überschritten!",
        Language::ES => "¡Se ha superado el límite de paquetes!",
        Language::FA => "آستانه بسته فراتر رفت!",
        Language::FR => "Le seuil de paquet a été atteint!",
        Language::IT => "Soglia di pacchetti superata!",
        Language::KO => "패킷 임계값 초과!",
        Language::PL => "Próg pakietów przekroczony!",
        Language::PT => "Limite de pacotes excedido!",
        Language::RO => "Prag de pachete depășit!",
        Language::RU => "Порог по числу пакетов превышен!",
        Language::TR => "Paket eşik değeri aşıldı!",
        Language::UK => "Ліміт пакетів перевищено!",
        Language::ZH => "达到设定的数据包数量阈值!",
    }
}

pub fn packets_exceeded_value_translation(language: Language, value: u32) -> String {
    match language {
        // TODO: Sorting
        Language::EN => match value {
            1 => "1 packet has been exchanged".to_owned(),
            npackets => format!("{npackets} packets have been exchanged"),
        },
        Language::IT => match value {
            1 => "1 pacchetto è stato scambiato".to_owned(),
            npackets => format!("{npackets} pacchetti sono stati scambiati"),
        },
        Language::FR => match value {
            1 => "1 paquet a été échangé".to_owned(),
            npackets => format!("{npackets} paquets ont été échangés"),
        },
        Language::ES => format!("{value} paquete/s han sido intercambiado/s"),
        Language::PL => format!("Wymieniono {value} pakietów"),
        Language::DE => format!("{value} Pakete wurden ausgetauscht"),
        Language::UK => format!("Обміняно {value} пакетів"),
        Language::ZH => format!("已交换数据包 {value}"),
        Language::RO => format!("au fost transferate {value} pachete"),
        Language::KO => format!("패킷 {value} 가 교환되었습니다"),
        Language::TR => format!("{value} paket aktarıldı"),
        Language::RU => format!("{value} пакет(ов) обмена информацией"),
        Language::PT => match value {
            1 => "Foi trocado 1 pacote".to_owned(),
            npackets => format!("Foram trocados {npackets} pacotes"),
        },
        Language::FA => format!("{value} بسته مبادله شده است"),
    }
}

pub fn favorite_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from favorites!",
        Language::DE => "Neue Daten mit den Favoriten ausgetauscht!",
        Language::ES => "¡Nuevos datos intercambiados de favoritos!",
        Language::FA => "مبادله داده جدید از پسندیده ها!",
        Language::FR => "Nouvel échange de donnée depuis un favori!",
        Language::IT => "Nuovi dati scambiati dai preferiti!",
        Language::KO => "즐겨찾기에서 새 데이터 교환",
        Language::PL => "Nowe dane wymienione z ulubionych!",
        Language::PT => "Novos dados trocados dos favoritos!",
        Language::RO => "Date noi transferate de la favorite!",
        Language::RU => "Новый обмен данными в избранных соедиениях!",
        Language::TR => "Favorilerden yeni veri aktarıldı!",
        Language::UK => "Нові дані обміняно з улюблених!",
        Language::ZH => "收藏夹内的连接有新活动!",
    }
}

pub fn no_notifications_set_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => "You haven't enabled notifications yet!\n\n\
                                 After you will enable them, this page will display a log of your notifications\n\n\
                                 You can enable notifications from settings:",
        Language::IT => "Non hai ancora abilitato le notifiche!\n\n\
                                Dopo che le avrai abilitate, questa pagina mostrerà una collezione delle tue notifiche\n\n\
                                Puoi abilitare le notifiche dalle impostazioni:",
        Language::FR => "Vous n'avez pas activé les notifications!\n\n\
                                    Une fois activées, cette page affichera le journal des notifications\n\n\
                                    Vous pouvez les activer dans les paramètres:",
        Language::ES => "¡Aún no has activado las notificaciones!\n\n\
                                 Después de activarlas, esta página mostrará un registro de sus notificaciones\n\n\
                                 Puedes activar las notificaciones desde los ajustes:",
        Language::PL => "Nie włączyłeś jeszcze powiadomień!\n\n\
                                 Po ich włączeniu, ta strona wyświetli dziennik twoich powiadomień\n\n\
                                 Możesz włączyć powiadomienia w ustawieniach:",
        Language::DE => "Benachrichtigungen wurden noch nicht aktiviert!\n\n\
                         Nachdem du sie aktiviert hast, wird diese Seite eine Liste deiner Benachrichtigungen anzeigen\n\n\
                         Du kannst die Benachrichtigungen in den Einstellungen aktivieren:",
        Language::UK => "Повідомлення не активовані!\n\n\
                                 Після їх активації, на цій сторінці побачиш список своїх повідомлень\n\n\
                                 Можеш вимкнути повідомлення в налаштуваннях:",
        Language::ZH => "您还没有设定任何通知!\n\n\
                                 启用它们后，此页面将显示您的通知日志\n\n\
                                 您可以从设置中设定:",
        Language::RO => "Încă nu ați activat notificările!\n\n\
                                 După ce le veți activa, această pagină va afișa un jurnal al notificărilor dvs\n\n\
                                 Puteți activa notificările din setări:",
        Language::KO => "아직 알림을 활성화하지 않았습니다!\n\n\
                                 활성화로 설정하면 이 페이지에 알림 로그가 표시됩니다\n\n\
                                 설정에서 알림을 활성화할 수 있습니다:",
        Language::TR => "Henüz bildirimleri etkinleştirmedin!\n\n\
                         Etkinleştirdikten sonra bu sayfada bildirimlerine ait kütüğü görebilirsin\n\n\
                         Bildirimleri, ayarlardan etkinleştirebilirsin:",
        Language::RU => "Уведомления пока не настроены!\n\n\
                         После настройки, эта страница будет показывать журнал уведомлений\n\n\
                         Вы можете включить уведомления в настройках:",
        Language::PT => "Ainda não ativou as notificações!\n\n\
                                Depois de ativá-las, esta página irá mostrar um registo das suas notificações\n\n\
                                Pode ativar as notificações nas definições:",
        Language::FA => "شما هنوز اعلان ها را فعال نکرده اید!\n\n\
                                 پس از آنکه آن ها را فعال کنید، این صفحه یک کارنامه از اعلان های شما را نمایش خواهد داد\n\n
                                 شما می توانید اعلان ها را از پیکربندی فعال کنید:",
    })
}

pub fn no_notifications_received_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        // TODO: Sorting
        Language::EN => {
            "Nothing to see at the moment...\n\n\
                                 When you will receive a notification, it will be displayed here"
        }
        Language::IT => {
            "Nulla da vedere al momento...\n\n\
                                Quando riceverai una notifica, essa verrà mostrata qui"
        }
        Language::FR => {
            "Rien à voir pour le moment...\n\n\
                                 Lorsque vous recevrez une notification, elle s'affichera ici"
        }
        Language::ES => {
            "Nada que ver por el momento...\n\n\
                                 Cuando reciba una notificación, aparecerá aquí"
        }
        Language::PL => {
            "Nic do wyświetlenia w tej chwili...\n\n\
                                 Gdy otrzymasz powiadomienie, pojawi się ono tutaj"
        }
        Language::DE => {
            "Im Moment nichts zu sehen...\n\n\
                                 Wenn du eine Benachrichtigung erhälst, wird sie hier angezeigt"
        }
        Language::UK => {
            "Немає що показати в даний момент...\n\n\
                                 Коли отримаєш повідомлення, побачиш його тут"
        }
        Language::ZH => {
            "还没有任何通知...\n\n\
                                 当您收到通知时，它会显示在这里"
        }
        Language::RO => {
            "Nimic de văzut momentan...\n\n\
                                 Când veți primi o notificare, aceasta va fi afișată aici"
        }
        Language::KO => {
            "현재는 볼 것이 없습니다...\n\n\
                                 알림을 받으면 여기에 표시됩니다"
        }
        Language::TR => {
            "Şu an görecek bir şey yok...\n\n\
                         Bildirim aldığınız zaman burada gözükecektir"
        }
        Language::RU => {
            "Нечего показывать в текущий момент...\n\n\
                                Когда прийдут уведомления, они будут показаны тут"
        }
        Language::PT => {
            "Nada para ver neste momento...\n\n\
                                Quando receber uma notificação, ela será mostrada aqui"
        },
        Language::FA => {
            "در حال حاضر هیچ چیزی برای دیدن نیست...\n\n\
                                 وقتی شما اعلانی دریافت می کنید، در اینجا نمایش داده خواهد شد"
        }
    })
}

pub fn only_last_30_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the last 30 notifications are displayed",
        Language::DE => "Nur die letzten 30 Benachrichtigungen werden angezeigt",
        Language::ES => "Sólo se muestran las últimas 30 notificaciones",
        Language::FA => "تنها ۳۰ اعلان آخر نمایش داده شده اند",
        Language::FR => "Seulement les 30 dernières notifications sont affichées",
        Language::IT => "Solo le ultime 30 notifiche sono mostrate",
        Language::KO => "최근 30개의 알림만 표시됩니다",
        Language::PL => "Wyświetlane jest tylko 30 ostatnich powiadomień",
        Language::PT => "São mostradas apenas as últimas 30 notificações",
        Language::RO => "Sunt afișate doar ultimele 30 de notificări",
        Language::RU => "Тут показываются только последние 30 уведомлений",
        Language::TR => "Sadece son 30 bildirim gösterilmektedir",
        Language::UK => "Можеш побачити лише 30 останніх повідомлень",
        Language::ZH => "仅显示最近 30 条通知",
    }
}
