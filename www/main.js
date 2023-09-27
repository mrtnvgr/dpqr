window.onload = function() {
    if ("pass" in localStorage) {
        document.querySelector("p.pass").textContent = localStorage.getItem("pass");
        document.querySelector("div#results").classList.remove("hidden");
    } else {
        document.querySelector("div#form").classList.remove("hidden");
    }
}

function submit() {
    let submit_button = document.querySelector("button.submit");

    submit_button.disabled = true;

    const old_button_text = submit_button.textContent;
    submit_button.textContent = "Подождите...";

    const inputs = document.getElementsByTagName("input");
    
    let values = [];
    for (let i = 0; i < inputs.length; i++) {
        let value = inputs[i].value;

        // Safety wheels
        if (value.includes(",")) {
            value = "";
        }

        values.push(value.toLowerCase());
    }

    const host = window.location.host;
    const protocol = window.location.protocol;
    const response = fetch(`${protocol}//${host}/api/submit`, {
        method: "POST",
        body: values.join(),
    }).then(response => {
        if (response.status === 200) {
            response.text().then(pass => {
                document.querySelector("p.pass").textContent = pass;
                document.querySelector("div#form").classList.add("hidden");
                document.querySelector("div#results").classList.remove("hidden");
                localStorage.setItem("pass", pass);
            });

            submit_button.disabled = false;
            submit_button.textContent = old_button_text;
        } else {
            submit_button.classList.toggle("error");

            submit_button.textContent = "Неправильный ответ";
            setTimeout(function() {
                submit_button.textContent = old_button_text;
                submit_button.classList.toggle("error");
                submit_button.disabled = false;
            }, 2000);
        }
    });
}
