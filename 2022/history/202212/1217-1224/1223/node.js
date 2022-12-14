const assert = require('node:assert/strict');

assert.deepEqual([[[1, 2, 3]], 4, 5], [[[1, 2, '3']], 4, 5]);

const assert = require('node:assert');

const tracker = new assert.CallTracker();

function func() { }

// callsfunc() must be called exactly 1 time before tracker.verify().
const callsfunc = tracker.calls(func, 1);

callsfunc();

// Calls tracker.verify() and verifies if all tracker.calls() functions have
// been called exact times.
process.on('exit', () => {
   tracker.verify();
});

const assert = require('node:assert');

// Creates call tracker.
const tracker = new assert.CallTracker();

function func() { }

// Returns a function that wraps func() that must be called exact times
// before tracker.verify().
const callsfunc = tracker.calls(func);

const tracker = new assert.CallTracker();

function func() { }
const callsfunc = tracker.calls(func);
callsfunc(1, 2, 3);

assert.deepStrictEqual(tracker.getCalls(callsfunc),
   [{ thisArg: this, arguments: [1, 2, 3] }]);

app.param(['id', 'page'], function (req, res, next, value) {
   console.log('CALLED ONLY ONCE with', value)
   next()
})

app.get('/user/:id/:page', function (req, res, next) {
   console.log('although this matches')
   next()
})

app.get('/user/:id/:page', function (req, res) {
   console.log('and this matches too')
   res.end()
})

//
var express = require('express')
var app = express()

// customizing the behavior of app.param()
app.param(function (param, option) {
   return function (req, res, next, val) {
      if (val === option) {
         next()
      } else {
         next('route')
      }
   }
})

// using the customized app.param()
app.param('id', 1337)

// route to trigger the capture
app.get('/user/:id', function (req, res) {
   res.send('OK')
})

app.listen(3000, function () {
   console.log('Ready')
})

///
app.param(function (param, validator) {
   return function (req, res, next, val) {
      if (validator(val)) {
         next()
      } else {
         next('route')
      }
   }
})

app.param('id', function (candidate) {
   return !isNaN(parseFloat(candidate)) && isFinite(candidate)
})

///
var app = express()
var blog = express()
var blogAdmin = express()

app.use('/blog', blog)
blog.use('/admin', blogAdmin)

console.dir(app.path()) // ''
console.dir(blog.path()) // '/blog'
console.dir(blogAdmin.path()) // '/blog/admin'

//app is mounted on /blog
blogAdmin.on('mount', function (parent) {
   console.log('Admin Mounted')
   console.log(parent) // refers to the parent app
}
)

//app?????????blogAdmin??????????????????????????????
blogAdmin.get('/', function (req, res) {
   res.send('Admin Homepage')
}
)
//parseFloat()???????????????????????????????????????????????????
app.param('id', function (candidate) {

   return !isNaN(parseFloat(candidate)) && isFinite(candidate)
}
)

//param()???????????????????????????????????????
app.param('id', function (req, res, next, id) {
   console.log('CALLED ONLY ONCE')
   next()
}
)
//next()?????????????????????????????????????????????
app.get('/user/:id', function (req, res, next) {
   console.log('although this matches')
   next()
}
)

//isFinite()???????????????????????????????????????????????????????????????
app.param('id', function (candidate) {
   return !isNaN(parseFloat(candidate)) && isFinite(candidate)
}
)

