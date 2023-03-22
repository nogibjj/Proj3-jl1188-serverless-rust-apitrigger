# rust-integer-to-roman-aws-lambda
An AWS lambda function in Rust to convert integer into Roman

## Steps to Run

* `make format` to format code
* `make lint` to lint it
* `make realease-arm` to build with arm
* `make deploy` to deploy to AWS (make sure correctly log in to AWS account first)
* `make invoke` to invoke the lambda function with argument 27

```Working Demo
(.venv) @selinaes ➜ /workspaces/wk5-Mini-Rust-jl1188/roman-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"num": 27}' \
                --output-format json \
                roman-lambda
{
  "msg": "Convert 27 to Roman: XXVII.",
  "req_id": "0352ce59-a609-46f1-88d4-102b17c073fe"
}

```
![AWS-Lambda-Running-pic](https://github.com/nogibjj/wk5-Mini-Rust-jl1188/blob/main/roman-lambda/aws-roman-lambda.jpeg?raw=true)


## References

* [rust-aws-lambda](https://github.com/noahgift/rust-mlops-template/tree/main/rust-aws-lambda)
