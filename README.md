# RustySchedule

A reminder app.

Add binary to `PATH` and run `schedule run` to set reminders to active.

## Example Save
A json file should be placed under `C:\Users\[USER]\AppData\Roaming\Rusty Notifier\data`:
```json
{
    "reminders": {
        "9:00:00":[{"title":"JOB","content":"OH NOES! IT'S JOB TIME ZZZ..."}],
        "17:00:00":[{"title":"WORK OUT","content":"HEAD TO THE GYM!"}],
        "20:00:00":[
            {"title":"PROGRAM","content":"WORK ON YOUR SIDE-PROJECTS","weekdays":["Mon", "Thu"]},
            {"title":"KOREAN","content":"LEARN AND PRACTICE KOREAN","weekdays":["Tue", "Fri", "Sun"],"open":"C:\\Users\\[USER]\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Anki.lnk"},
            {"title":"WRITING","content":"WRITE YOUR NOVEL","weekdays":["Wed", "Sat"]}
        ],
        "21:30:00":[{"title":"DAILY KOREAN","content":"LEARN AND PRACTICE KOREAN","open":"C:\\Users\\[USER]\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Anki.lnk"}],
        "22:00:00":[{"title":"BEDTIME","content":"HEAD TO BED ZZZ..."}]
    }
}
```

`open` can be a URL or point to an application. It can be an array or string.
Once the reminder hits, it will open the URL in your browser or open the application.
NOTE: make sure the URL includes the `www.`
