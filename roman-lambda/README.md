# rust-integer-to-roman-aws-lambda
An AWS lambda function in Rust to convert integer into Roman

## Steps to Run

* `make format` to format code
* `make lint` to lint it
* `make realease-arm` to build with arm
* `make deploy` to deploy to AWS (make sure correctly log in to AWS account first)
* `make invoke` to invoke the lambda function with a payload of name Cheesecake and price 7

```Working Demo
(.venv) @selinaes ➜ /workspaces/Proj3-jl1188-serverless-rust-stepfunction/roman-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name":"Cheesecake","price":7}' \
                --output-format json \
                roman-lambda
{
  "msg": "Your Tatte dish Cheesecake is priced at $7, in Roman: VII.",
  "req_id": "6784126b-a9e7-4c5b-8177-8c3db0656aca"
}

```
![AWS-Lambda-Running-pic](https://github.com/nogibjj/wk5-Mini-Rust-jl1188/blob/main/roman-lambda/aws-roman-lambda.jpeg?raw=true)


## References

* [rust-aws-lambda](https://github.com/noahgift/rust-mlops-template/tree/main/rust-aws-lambda)
