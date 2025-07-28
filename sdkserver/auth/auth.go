package auth

import "net/http"

func AddHandlers(h *http.ServeMux) {
	h.HandleFunc("/account/risky/api/check", riskyCheckHandler)
	h.HandleFunc("/hkrpg_global/combo/granter/login/v2/login", tokenLoginHandler)
	h.HandleFunc("/hkrpg_global/account/ma-passport/api/appLoginByPassword", passwordLoginHandler)
}

func riskyCheckHandler(w http.ResponseWriter, r *http.Request) {
	response := `{
		"data": {},
		"message": "OK",
		"retcode": 0
	}`
	w.Header().Set("Content-Type", "application/json")
	w.Write([]byte(response))
}

func tokenLoginHandler(w http.ResponseWriter, r *http.Request) {
	response := `{
		"data": {
			"account_type": 1,
			"combo_id": "1",
			"combo_token": "x",
			"data": "{\"guest\":false}",
			"heartbeat": false,
			"open_id": "1"
		},
		"message": "OK",
		"retcode": 0
	}`
	w.Header().Set("Content-Type", "application/json")
	w.Write([]byte(response))
}

func passwordLoginHandler(w http.ResponseWriter, r *http.Request) {
	response := `{
		"data": {
			"bind_email_action_ticket": "",
			"ext_user_info": {
				"birth": "0",
				"guardian_email": ""
			},
			"reactivate_action_token": "",
			"token": {
				"token_type": 1,
				"token": "x"
			},
			"user_info": {
				"account_name": "yulian",
				"aid": "1",
				"area_code": "**",
				"token": "x",
				"email": "yuvlian@naver.com",
				"is_email_verify": "1",
				"country": "ID"
			}
		},
		"message": "OK",
		"retcode": 0
	}`
	w.Header().Set("Content-Type", "application/json")
	w.Write([]byte(response))
}
