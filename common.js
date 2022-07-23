// native functions
// const invoke = window.__TAURI__.invoke
// const sendNotification = window.__TAURI__.notification.sendNotification
// const isLoggedIn = () => invoke('is_logged_in')
// const logIn = token => invoke('log_in', { token })
// const listRunners = () => invoke('list_runners')
// const listJobsForRunner = id => invoke('list_jobs_for_runner', { runnerId: id })
// const retryJob = (projectId, jobId) => invoke('retry_job', { projectId, jobId })

const API = 'https://gitlab.com/api/v4'

const jsonify = res => res.json()

const isLoggedIn = async () => {
	const token = localStorage.getItem('gitlab-jobs-token')
	// TODO: check stuff?
	return token != null
}

const logIn = async token => {
	const res = await fetch(API + '/user', {
		headers: {
			'PRIVATE-TOKEN': token
		}
	})

	if (res.ok) {
		localStorage.setItem('gitlab-jobs-token', token)
		return true
	}

	return false
}

const headers = () => ({ headers: { 'PRIVATE-TOKEN': localStorage.getItem('gitlab-jobs-token') } })

const listRunners = () => fetch(API + '/runners', headers()).then(jsonify)
const listJobsForRunner = id => fetch(API + `/runners/${id}/jobs?order_by=id`, headers()).then(jsonify)
const retryJob = () => {}

const sendNotification = async ({ title, body }) => {
    if (Notification.permission !== 'granted') {
        const result = await Notification.requestPermission()
        if (result !== 'granted') {
            console.error('did not get permission :(', result)
            return
        }
    }

    return new Notification(title, { body })
}

// shit helpers
const $ = (...args) => {
    if (args.length === 1) return document.querySelector(args[0])
    return args[0].querySelector(args[1])
}
const $$ = document.querySelectorAll.bind(document)

function initTemplate(template, data) {
    const fragment = template.cloneNode(true)
    Object.keys(data).forEach(key => {
        const field = fragment.querySelector('.' + key)
        const value = data[key]
        if (typeof value === 'string') {
            field.textContent = value
        } else if (typeof value === 'number') {
            field.textContent = value.toString()
        } else if (value.link) {
            const a = field.querySelector('a')
            a.href = value.link
            a.textContent = value.text
        }
    })
    return fragment
}
