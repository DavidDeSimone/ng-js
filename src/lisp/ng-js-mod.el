(require 'ng-js)
(require 'filenotify)

(setq ng-js-temp (make-temp-file "ng-js"))
(ng-js-eval (format "ngjsSetFileMarker('%s')" ng-js-temp))
(defun execute-lisp (event)
    (with-temp-buffer
        (insert (ng-js-drain))
        (eval-buffer)
    ))

(setq desc (file-notify-add-watch
  ng-js-temp '(change attribute-change) 'execute-lisp))