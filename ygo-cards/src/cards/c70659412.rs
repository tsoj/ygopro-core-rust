//! Psychic Omnibuster
use super::prelude::*;

pub const ID: u32 = 70659412;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner Psychic monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_PSYCHIC),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_PSYCHIC, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Look at 1 random card in your opponent's hand, and if it is the declared card type, apply these effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e,tp,eg,ep,ev) return ep==1-tp and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)&(LOCATION_MZONE|LOCATION_SZONE)>0 end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            if a.ep != 1 - a.tp {
                return false;
            }
            let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
            loc[0].int() & (LOCATION_MZONE | LOCATION_SZONE) as i64 > 0
        })),
    );
    // e1:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e1:SetTarget(s.looktg)
    effect::set_target(d, e1, Some(cost_target(looktg)));
    // e1:SetOperation(s.lookop)
    effect::set_operation(d, e1, Some(operation(lookop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //   s.declared_types={}
    //   s.declared_types[0]=0
    //   s.declared_types[1]=0
    //   aux.AddValuesReset(function()
    //     s.declared_types[0]=0
    //     s.declared_types[1]=0
    //   end)
    // end)
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        aux::add_values_reset(d, reset_declared_types);
    });
}
/// The `aux.AddValuesReset` callback: `s.declared_types[0]=0`, `s.declared_types[1]=0`.
fn reset_declared_types(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.looktg(e,tp,...,chk)`
fn looktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local declared_types=s.declared_types[tp]
    let declared_types = s_var(d, ID, tp as u64).int();
    // if chk==0 then return declared_types~=(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP) and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return declared_types != (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP) as i64 && duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CARDTYPE as u64);
    // local op=Duel.SelectEffect(tp,
    //   {declared_types&TYPE_MONSTER==0,DECLTYPE_MONSTER},
    //   {declared_types&TYPE_SPELL==0,DECLTYPE_SPELL},
    //   {declared_types&TYPE_TRAP==0,DECLTYPE_TRAP})
    let op = duel::select_effect(
        d,
        tp,
        &[
            (declared_types & TYPE_MONSTER as i64 == 0, DECLTYPE_MONSTER as u64),
            (declared_types & TYPE_SPELL as i64 == 0, DECLTYPE_SPELL as u64),
            (declared_types & TYPE_TRAP as i64 == 0, DECLTYPE_TRAP as u64),
        ],
    )
    .unwrap_or(0);
    // local typ=1<<(op-1)
    let typ = 1i64 << (op - 1);
    // s.declared_types[tp]=declared_types|typ
    set_s_var(d, ID, tp as u64, Value::Int(declared_types | typ));
    // Duel.SetTargetParam(typ)
    duel::set_target_param(d, typ as u32);
    true
}
/// `s.lookop(e,tp,...)`
fn lookop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) == 0 {
        return;
    }
    // local rc=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1):GetFirst()
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    let rg = group::random_select(d, &g, tp, 1);
    let rc = group::get_first(d, &rg).nil();
    // Duel.ConfirmCards(tp,rc)
    duel::confirm_cards(d, tp, rc);
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
    // local typ=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let typ = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // if not rc:IsType(typ) then return end
    if !card::is_type(d, rc, typ) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local desc=(typ==TYPE_MONSTER and 3010) or (typ==TYPE_SPELL and 3011) or (typ==TYPE_TRAP and 3012)
        let desc = if typ == TYPE_MONSTER {
            3010
        } else if typ == TYPE_SPELL {
            3011
        } else if typ == TYPE_TRAP {
            3012
        } else {
            0
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(desc)
        effect::set_description(d, e1, desc);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetValue(function(e,te) return te:IsActiveType(typ) end)
        effect::set_value_fn(
            d,
            e1,
            lua_fn(move |d, args: Args| {
                let te = args.e(1);
                effect::is_active_type(d, te, typ)
            }),
        );
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if rc:IsAbleToRemove() then
    if card::is_able_to_remove(d, rc, None, None, None) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // aux.RemoveUntil(rc,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end)
        aux::remove_until(
            d,
            rc,
            Some(POS_FACEUP as u8),
            REASON_EFFECT | REASON_TEMPORARY,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
                true
            })),
            None,
            None,
            None,
            None,
            None,
        );
    }
}
