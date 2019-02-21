//Add leave event
      leaveEvent: event => {
        console.log('leaving event')
        this.actions.setEvent(event)
        this.makeHolochainCall(`${instanceID}/event/leave_event`, { event_address: event.id }, (result) => {
          console.log('left event', result)
        })
      },
