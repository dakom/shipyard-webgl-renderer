const fs = require("fs");

fs.readFile("./public/index.html", "utf-8", (err, index) => {
    if(err) {
        throw err;
    } else {
        fs.readFile("./public/404.html", "utf-8", (err, missing) => {
            if(err) {
                throw err;
            } else {
                rewrite(index, missing);
            }
        });
    }
});

function rewrite(index, missing) {
    const roots = ["css", "media", "wasm"];
    for(const root of roots) {
        index = index.replaceAll(`"/${root}`, `"/awsm-renderer/${root}`);
        missing = missing.replaceAll(`"/${root}`, `"/awsm-renderer/${root}`);
    }

    fs.writeFile("./public/prod-index.html", index,
      {
        encoding: "utf8",
        flag: "w",
      },
        (err) => {
            if(err) {
                throw err; 
            } else {

                fs.writeFile("./public/prod-404.html", missing,
                  {
                    encoding: "utf8",
                    flag: "w",
                  },
                  (err) => {
                      if(err) {
                        throw err; 
                      }
                  });
            }
      });


    // now run
    // mv ./public/prod-index.html ./public/index.html
    // mv ./public/prod-404.html ./public/404.html
}
