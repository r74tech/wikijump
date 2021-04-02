{loadmacro set="Forms"}

		<div class="error-block" id="ca-reg0-errors" style="display: none"></div>

		<form name="caform" action="dummy.html" method="get" onkeypress="return OZONE.utils.disableEnterKey(event)" id="createaccount-form0" onsubmit="return false">
			<table class="form">
				<tr>
					<td>
						{t}Your screen name{/t}:
					</td>
					<td>
						<input class="text" type="text" maxlength="50" size="25" name="name"/>
						<div class="sub">
							{t 1=$SERVICE_NAME}That is your new name at %1.{/t}
						</div>
					</td>
				</tr>
				<tr>
					<td>
						{t}Your email address{/t}:
					</td>
					<td>
						<input class="text" type="text" maxlength="50" size="25" name="email"/>
					</td>
				</tr>
				<tr>
					<td>
						{t}Preferred language{/t}:
					</td>
					<td>
						<input type="radio" name="language" value="en" id="new-site-lang-en"> <label for="new-site-lang-en">{t}English{/t}</label>
						&nbsp;&nbsp;&nbsp;&nbsp;
						<input type="radio" name="language" value="pl" id="new-site-lang-pl"> <label for="new-site-lang-pl">{t}Polish{/t}</label>

					</td>
				</tr>

		{*	</table>
			<h2>Account access password </h2>
			<table  class="form">	*}
				<tr>
					<td>
						{t}Password{/t}:
					</td>
					<td>
						 <input class="text" type="password" name="password" maxlength="1024" size="15"/>
						 <div class="sub">
						 	{t}Between 8 and 1024 characters{/t}.
						 </div>
					</td>
				</tr>
				<tr>
					<td>
						{t}Password (repeat){/t}:
					</td>
					<td>
						 <input class="text" type="password" name="password2" maxlength="1024" size="15"/>
					</td>
				</tr>
				<tr>
					<td>
						{t}Are you a human?{/t}
					</td>
					<td>
						<script type="module" src="https://cdn.jsdelivr.net/npm/friendly-challenge@0.8.3/widget.module.min.js" async defer></script>
						<script nomodule src="https://cdn.jsdelivr.net/npm/friendly-challenge@0.8.3/widget.min.js" async defer></script>
						<div class="frc-captcha" data-sitekey="{$captchaSiteKey}"></div>
					</td>
				</tr>
				<tr>
					<td>
						{t}Please confirm:{/t}
					</td>
					<td>
						<input type="checkbox" name="tos" class="checkbox">

						<!-- TODO: De-Wikijump.com-ize - change -->
						{t 1=$URL_HOST}I have read and agree to the <a href="{$HTTP_SCHEMA}://%1/legal:terms-of-service"
						target="_blank">Terms of Service</a>.{/t}

					</td>

				</tr>
			</table>

		{*DEBUG: evcode: {$evcode}*}

	<div class="buttons">
		<input type="button" class="button" onclick="Wikijump.modules.CreateAccountStep1Module.listeners.cancel(event)" value="{t}cancel{/t}"/>
		<input type="button" class="button" onclick="Wikijump.modules.CreateAccountStep1Module.listeners.nextClick(event)" value="{t}next{/t}"/>
	</div>
</form>

