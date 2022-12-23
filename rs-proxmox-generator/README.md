# rs-proxmox-generator

We want to generate the rust proxmox api client from the API specification of the [apidoc.json](./apidoc.json).

## Naming conventions: paths & resources

The json datastructure of the [apidoc.json](./apidoc.json) allows us to flatten the tree. Indeed, once the specs
are flattened, we will range over them and generate the code.

Each function/structs name will be generated from the described pattern of the paths obtained from `*.children[].path`:
- We will replace slashes (`/`) by underscores (`_`). NB: snake_case will be converted to CamelCase for Struct & enum
names.
- We will replace curly braces by `xx`, e.g. `{node}` become `xxnodexx` or `Xxnodexx`.
  - Please notice the first letter of the name will not be capitalized in CamelCase.
  - This choice is made to respect the naming convention for paths. 
  - Indeed, we don't want to confuse `/{node}/` with `/xx/nodexx`.

Please note we will place each Resource into one file named after the naming convention for paths.

## Organisation of resources inside the crate

### 1. Getting started

As discussed in `Naming conventions: paths & resources`, we will organise the code by:
- Flattening the datastructure.
- Assigning each resource, corresponding to one specific path, to **one file**, named after the convention.  

Thus, each resource will declare structs, enums, associated functions, etc...
Before diving deeper into the discussion, maybe we should state why we are doing all this.

### 2. What is our goal?

- We want to provide a client for the Proxmox VE API. Thus, we need to be able to request each "path" of the API.
- Each path is made of one to several endpoint. These endpoints are characterized by the http method use to make the 
request, which can be GET, POST, PUT or DELETE.
- When sending a http request to any of these endpoints, we, not necessarily but in most cases, need to provide input 
data.
- Then, we get back a response from the API, and we might be interested in the deserializable data of the response.

Great, now we know where we're going, we can better plan how we want to organize the code of our resources.

### 3. Endpoints & Http methods

Remember we wanted to handle and request each endpoint, meaning that we need structs & functions to query each of these.

Let's say we have a path `/myexample` which have `GET` & `POST` endpoints available.
- We need a function `get` & function `post` returning us the deserialized data of the response.
- Therefor, the `struct` into which we'll deserialize the response data, needs to be defined. 
- Same for the input data, we need to define a struct out of which we can serialize the input data.

```rust
// In: src/__resource_cleaned_path__.rs

const API_PATH_FORMAT: &str = "__path__";

#[derive(Serialize)]
pub struct __Method__Input { ... }

#[derive(Deserialize)]
pub struct __Method__Output { ... }

pub async fn method(input: __Method__Input) -> Result<__Method__Output> { 
  ...
}
```

### 4. Dynamic paths

It's not that simple, some path are made out of dynamic ids like `{node}` or `{vmid}`.

We need to programmatically infer these from the data inside `API_PATH_FORMAT` & from the data inside `__Method__Input`

### 5. Enums

There is a non-negligible amount of input,output fields defining enum types.

The naming convention should reflect the concatenation of  `__Method__` `__Input_or_output__` `__Name_of_the_enum_field__`, e.g.:

```rust
pub enum GetInputMyEnum { ... }
...
pub enum PostOutputAnotherEnum { ... }
```

### 6. Error handling

We also need to handle erros from the response. We might tumble into authorization errors, input errors, connectivity 
issues... 

Therefor we should take care to clearly define & propagate errors.

### 7. Descriptions

The [apidoc.json](./apidoc.json) provides us with description for each endpoint & input,output fields.  
It's important to keep in mind and generate the docs for each structs & fields out of these description.

The same for the `format` description of each field.
