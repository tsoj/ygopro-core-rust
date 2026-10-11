//! Dark Magic Talisman
//! 黒魔術の護符
use super::prelude::*;

pub const ID: u32 = 71440209;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can activate this card the turn it was Set by revealing 1 Spellcaster monster in your hand
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e0:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e0:SetValue(function(e) e:SetLabel(1) end)
    effect::set_value_fn(d, e0, lua_fn(|d, a: Args| {
        effect::set_label(d, a.e(0), &[1]);
    }));
    // e0:SetCondition(function(e) return Duel.IsExistingMatchingCard(s.discostfilter,e:GetHandlerPlayer(),LOCATION_HAND,0,1,nil) end)
    effect::set_condition(d, e0, Some(lua_fn(|d, a: Args| {
        let p = effect::get_handler_player(d, a.e(0));
        duel::is_existing_matching_card(d, Some(&discostfilter), p, LOCATION_HAND, 0, 1, NO_EX)
    })));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Negate a monster effect that is activated in response to the activation of a card that mentions "Dark Magician" or its effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetCost(s.discost)
    effect::set_cost(d, e1, Some(cost_target(discost)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetCost(Cost.PayLP(2500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(2500))));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local chainlink=Duel.GetCurrentChain(true)-1
    let chainlink = duel::get_current_chain(d, true) - 1;
    // if not (chainlink>0 and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)) then return false end
    if !(chainlink > 0 && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)) {
        return false;
    }
    // local trig_eff=Duel.GetChainInfo(chainlink,CHAININFO_TRIGGERING_EFFECT)
    let trig_eff = duel::get_chain_info(d, chainlink as u8, &[CHAININFO_TRIGGERING_EFFECT as u32])[0].as_effect();
    // return trig_eff and trig_eff:GetHandler():ListsCode(CARD_DARK_MAGICIAN)
    match trig_eff {
        Some(te) => {
            let h = effect::get_handler(d, te).nil();
            card::lists_code(d, h, &[CARD_DARK_MAGICIAN as u32])
        }
        None => false,
    }
}
/// `s.discostfilter(c)`
fn discostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and not c:IsPublic()
    card::is_race(d, c, RACE_SPELLCASTER) && !card::is_public(d, c)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local label_obj=e:GetLabelObject()
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    // if chk==0 then label_obj:SetLabel(0) return true end
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    // if label_obj:GetLabel()>0 then
    if effect::get_label(d, label_obj) > 0 {
        // label_obj:SetLabel(0)
        effect::set_label(d, label_obj, &[0]);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        // local g=Duel.SelectMatchingCard(tp,s.discostfilter,tp,LOCATION_HAND,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&discostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    }
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // local code1,code2=re:GetHandler():GetOriginalCodeRule()
        let rh = effect::get_handler(d, a.re.nil()).nil();
        let codes = card::get_original_code_rule(d, rh);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
        // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsMonsterEffect() and re:GetHandler():IsOriginalCodeRule(code1,code2) end)
        let cond_codes = codes.clone();
        effect::set_condition(d, e1, Some(condition(move |d, a| {
            effect::is_monster_effect(d, a.re.nil()) && {
                let rch = effect::get_handler(d, a.re.nil()).nil();
                card::is_original_code_rule(d, rch, &cond_codes)
            }
        })));
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.Hint(HINT_CARD,0,id) Duel.NegateEffect(ev) end)
        effect::set_operation(d, e1, Some(operation(|d, a| {
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            duel::negate_effect(d, a.ev as u8);
        })));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.setconfilter(c,tp)`
fn setconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_DARK_MAGICIAN) and c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_code(d, c, &[CARD_DARK_MAGICIAN as u32]) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.setconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| setconfilter(d, c, tp), 1, NO_EX)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SSet(tp,c) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::s_set(d, a.tp, c, None, true);
    }
}
