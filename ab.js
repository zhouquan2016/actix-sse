const test_sse = async () => {
    for (let i = 0; i < 60000; i++) {
        const startTime = new Date().getTime();
        try {
            const url = `http://192.168.48.142:8080/sse?user_id=${i}`;
            // const url = `http://192.168.48.142:9090/sse/`;
            const resp = await fetch(url, {
            "headers": {
              "sec-ch-ua": "\"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"114\", \"Google Chrome\";v=\"114\"",
              "sec-ch-ua-mobile": "?0",
              "sec-ch-ua-platform": "\"Windows\"",
              "upgrade-insecure-requests": "1"
            },
            "referrerPolicy": "strict-origin-when-cross-origin",
            "body": null,
            "method": "GET"
          });
            console.log(i, resp.status, new Date().getTime() - startTime, "ms");
        }catch(e) {
            console.log(i, e);
        }
        
    }
    console.log("ok");
}
test_sse()

