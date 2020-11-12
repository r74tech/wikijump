

Wikijump.modules.AccountModule = {}

Wikijump.modules.AccountModule.vars = {
	currentId: null
}

Wikijump.modules.AccountModule.listeners = {
	clickMenu: function(e){
		var target = YAHOO.util.Event.getTarget(e);
		var id = target.id;
		target = target.parentNode;
		var list = target.getElementsByTagName("ul").item(0);
		if(!list){
			// means this is the link somewhere... at least should be.
			Wikijump.modules.AccountModule.utils.loadModule(id);
		} else{
			if(target.tagName.toLowerCase() != 'li') {return;}
			// toggle "selected" class
			if(YAHOO.util.Dom.hasClass(target,"selected")){
				var eff = new fx.Opacity(list, {duration: 200});
				eff.custom(1,0);
				tz = target;
				setTimeout('YAHOO.util.Dom.removeClass(tz,"selected")', 200);

			}else{
				YAHOO.util.Dom.addClass(target,"selected");
				var eff = new fx.Opacity(list, {duration: 200});
				eff.setOpacity(0);
				eff.custom(0,1);
			}
		}
	},

	editProfile: function(e){

	}
}

Wikijump.modules.AccountModule.callbacks = {
	menuClick: function(r){
		if(!Wikijump.utils.handleError(r)) {return;}
		$("account-area").innerHTML = r.body;
		OZONE.utils.formatDates($("account-area"));
	}
}
Wikijump.modules.AccountModule.utils = {
	loadModule: function(id){
		var mm = Wikijump.modules.AccountModule.mapping;
		var module = mm[id];
		if(module){
			// toggle current
			var currentId = Wikijump.modules.AccountModule.vars.currentId;
			if(currentId) {YAHOO.util.Dom.removeClass(currentId, "active");}
			Wikijump.modules.AccountModule.vars.currentId = id;
			YAHOO.util.Dom.addClass(id, "active");
			OZONE.ajax.requestModule(module, null, Wikijump.modules.AccountModule.callbacks.menuClick,
				null, {clearRequestQueue: true});

			// make sure the parent is unfolded (if is a list)
			var p = $(id).parentNode.parentNode.parentNode;

			var list = p.getElementsByTagName("ul").item(0);

			if(list && p.tagName.toLowerCase() == 'li' && !YAHOO.util.Dom.hasClass(p,"selected")){
				// unfold
				YAHOO.util.Dom.addClass(p,"selected");
				var eff = new fx.Opacity(list, {duration: 200});
				eff.setOpacity(0);
				eff.custom(0,1);
			}
		}
	}
}

Wikijump.modules.AccountModule.init = function(){

	YAHOO.util.Event.addListener("account-side", "click", Wikijump.modules.AccountModule.listeners.clickMenu);

	var mm = new Object();
	mm['am-welcome'] = "account/AccountWelcomeModule";
	mm['am-messages'] = "account/AccountMessagesModule";
	mm['am-notifications'] = "account/AccountNotificationsModule";
	mm['am-contacts'] = "account/contacts/AccountContactsModule";
	mm['am-profile'] = "account/AccountProfileModule";
	mm['am-adminof'] = "account/membership/AccountAdminOfModule";
	mm['am-moderatorof'] = "account/membership/AccountModeratorOfModule";
	mm['am-memberof'] = "account/membership/AccountMemberOfModule";
	mm['am-invitations'] = "account/membership/AccountInvitationsModule";
	mm['am-applications'] = "account/membership/AccountApplicationsModule";
	mm['am-recentcontrib'] = "userinfo/UserChangesModule";//"account/AccountRecentContribModule";
	mm['am-recentposts'] = "userinfo/UserRecentPostsModule";//"account/AccountRecentPostsModule";
	mm['am-stats'] = "account/AccountStatisticsModule";
	mm['am-settings'] = "account/AccountSettingsModule";
	mm['am-watched-changes'] = "account/watch/AWChangesModule";
	mm['am-watched-forum'] = "account/watch/AWForumModule";
	mm['am-watched-feed'] = "account/watch/AWFeedModule";
	mm['am-wiki-newsletters'] = "account/membership/AccountWikiNewslettersModule";
	mm['am-deletedsites'] = "account/membership/AccountDeletedSitesModule";

	Wikijump.modules.AccountModule.mapping = mm;

	OZONE.dom.onDomReady(function(){
		if(!$("account-area")){
			return;
		}

		var startPage = "am-welcome";
		if(window.accountStartPage){
			startPage = 'am-'+accountStartPage;
		}
		// on DOM complete!!!

		Wikijump.modules.AccountModule.utils.loadModule(startPage);
	}, "dummy-ondomready-block");

}

Wikijump.modules.AccountModule.init();
