(defun file-to-string (file)
  "File to string function"
  (with-temp-buffer
    (insert-file-contents file)
    (buffer-string)))

(setq test-js-string (file-to-string "tests/test.js"))
(print (ng-js-eval test-js-string))
;; (kill-emacs)
