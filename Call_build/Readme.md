<h1>Llamadas a Build Call</h1>

<h2>Compilar Flipper , desplegar en cadena  </h2>
token_contract_address: Es la direccion de contrato de Flipper 
<h2>Compilar tokenSend , desplegar </h2>


<h2>Build Call:</h2>
🦑Hay dos Funciones: <br>
<strong> pub fn flip_contract_callbuildeer_sin_gas(&mut self,token_contract_address: AccountId )-> () (</strong> <br>
---->  llama a la fn flip() del contrato flipper  <br>

<strong> pub fn get_flip2_buill_call(&mut self, token_contract_address: AccountId, account_id: AccountId )-> i32 (</strong> <br>
---->  llama a la fn get() del contrato flipper <br>

<strong> pub fn getBalanceOfFromUserOfVault(&mut self, token_contract_address: AccountId,account_id: AccountId)->Balance (</strong> <br>
---->  llama a la fn totalsupplypricesByCount() del contrato psp22Token1 <br>

<strong>   pub fn transferToFromVault(&mut self, token_contract_address: AccountId, account_id: AccountId, amount: Balance, _data: Vec<u8>,)->()(</strong> <br>
---->  llama a la fn transferTo() del contrato psp22Token1 <br>

Source: https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html
.CHEERS!
