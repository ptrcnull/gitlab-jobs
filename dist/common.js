// native functions
const invoke = window.__TAURI__.invoke
const isLoggedIn = () => invoke('is_logged_in')
const logIn = token => invoke('log_in', { token })
const listRunners = () => invoke('list_runners')
const listJobsForRunner = id => invoke('list_jobs_for_runner', { runnerId: id })
const retryJob = (projectId, jobId) => invoke('retry_job', { projectId, jobId })

// shit helpers
const $ = (...args) => {
    if (args.length === 1) return document.querySelector(args[0])
    return args[0].querySelector(args[1])
}
const $$ = document.querySelectorAll.bind(document)

function initTemplate(template, data) {
    const fragment = template.content.cloneNode(true)
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