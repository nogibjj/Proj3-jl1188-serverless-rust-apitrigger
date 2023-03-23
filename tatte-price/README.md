# rust-tatte-price-aws-lambda-with-API-trigger
An AWS lambda function in Rust that tells you the price of a dish in tatte (Boston brunch place) given the dish name.

## Steps to Build the Serverless Rust Lambda Function with API Trigger

* `python3 -m venv ~/.venv`; `source ~/.venv/bin/activate`; `pip3 install cargo-lambda` to install cargo-lambda
* `cargo lambda new <project name>` to create a new lambda project (will ask to select whether to use API Gateway or not)
* Write the code in `src/main.rs`
* `cargo lambda build --release --target x86_64-unknown-linux-gnu.2.17` to build with x86_64-unknown-linux. Make sure to add the ".2.17" because "Amazon Linux 1 uses glibc version 2.17, while Rust binaries need glibc version 2.18 or later by default. However, with Cargo Lambda, you can specify a different version of glibc." -- as explained in the "Rust Runtime for AWS Lambda - Building and deploying your Lambda functions" reference below.
* `touch serverless.yaml` inside the project folder to create a serverless.yaml file
* Write necessary configurations in `serverless.yaml` file. For example, make sure to use the runtime: provided.al2. Define an HTTP event for the function, including the path with a param.
* Steps to do if not already done:
```
sudo apt install curl
curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g serverless
curl -o- -L https://slss.io/install | VERSION=2.72.2 bash
```
* `serverless deploy` to deploy to AWS (make sure correctly log in to AWS account first. You may use the "AWS Toolkit" extension in VS Code, then select "Connect to AWS" - "Use IAM Credentials" - then paste in the access key and secret key from the AWS IAM console.)
* When the GET endpoint is given, use curl to access the endpoint, ex. `curl -i https://pggapjrkvd.execute-api.us-east-1.amazonaws.com/dev/tatte/Cheesecake/price`
* If encountered things like "Internal Server Error", try debugging by looking into the AWS Lambda Function Console - Monitor - Logs

## Run it
* `curl -i https://pggapjrkvd.execute-api.us-east-1.amazonaws.com/dev/tatte/<dish_name>/price`

Dish name list:
* Cheesecake
* Breakfast_Sandwich
* French_Toast
* Avocado_Tartines
* Traditional_Shakshuka
* Chicken_Pita
* Chicken_Salad

```Working Demo
(.venv) @selinaes ➜ /workspaces/Proj3-jl1188-serverless-rust-stepfunction (main) $ curl -i https://pggapjrkvd.execute-api.us-east-1.amazonaws.com/dev/tatte/Chicken_Pita/price
HTTP/2 200 
content-type: application/json
content-length: 34
date: Wed, 22 Mar 2023 19:02:34 GMT
x-amzn-requestid: f2044bad-c108-4279-8f06-65f8f8c085c2
x-amz-apigw-id: CMmPnHxDoAMF3nw=
x-amzn-trace-id: Root=1-641b50ca-64d871a95ef5c2ea5065818f;Sampled=0
x-cache: Miss from cloudfront
via: 1.1 5d4199dbed922d7847172f5631f32dbc.cloudfront.net (CloudFront)
x-amz-cf-pop: IAD12-P2
x-amz-cf-id: cit-bIimIv5GhDSXJpV6fo5_YZCa5QGNvlcJHjCJetb7u3mjhjbwuw==

{"name":"Chicken_Pita","price":12}

```
![AWS-Lambda-Running-pic](https://github.com/nogibjj/wk5-Mini-Rust-jl1188/blob/main/roman-lambda/aws-roman-lambda.jpeg?raw=true)


## References

* [rust-aws-lambda](https://github.com/noahgift/rust-mlops-template/tree/main/rust-aws-lambda)
* [maxday-yt-lambda-rust](https://github.com/maxday/yt-lambda-rust)
* [RUST on AWS Lambda | TDD live coding | Build a 🍕 API | No talking](https://www.youtube.com/watch?v=Idys2BAmqIU)
* [Rust Runtime for AWS Lambda - Building and deploying your Lambda functions](https://github.com/awslabs/aws-lambda-rust-runtime#12-build-your-lambda-functions)
* [/var/task/bootstrap: /lib64/libc.so.6: version `GLIBC_2.25' not found (required by /var/task/bootstrap) #455](https://github.com/awslabs/aws-lambda-rust-runtime/issues/455)
* [Serverless Framework](https://www.serverless.com/framework/docs/providers/aws/guide/events)
