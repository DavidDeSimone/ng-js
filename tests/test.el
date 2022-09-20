(defun file-to-string (file)
  "File to string function"
  (with-temp-buffer
    (insert-file-contents file)
    (buffer-string)))

(setq test-js-string (file-to-string "tests/test.js"))
(print (ng-js-eval test-js-string))

;; @TODO there may be a better way to do this.
(defun timer-kill ()
 (kill-emacs))
(run-with-timer 5 nil 'timer-kill)