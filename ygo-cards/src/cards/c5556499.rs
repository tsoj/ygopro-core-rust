//! Machina Fortress
use super::prelude::*;

pub const ID: u32 = 5556499;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(destroy_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetOperation(s.hdop)
    effect::set_operation(d, e3, Some(operation(hdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsDiscardable()
    card::is_race(d, c, RACE_MACHINE) && card::is_discardable(d, c, None, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let e = a.e(0);
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    let eff = card::get_card_effect(d, c, EFFECT_NECRO_VALLEY);
    // for _,te in ipairs(eff) do
    for te in eff {
        // local op=te:GetOperation()
        // if not op or op(e,c) then return false end
        if necro_blocks(d, te, e, c) {
            return false;
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil)
    let mut g = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND, 0, NO_EX);
    // if not c:IsAbleToGraveAsCost() then g:RemoveCard(c) end
    if !card::is_able_to_grave_as_cost(d, c) {
        group::remove_card(&mut g, &CardSet::from_card(c));
    }
    // if g:IsContains(c) then
    if group::is_contains(&g, c) {
        // eff={Duel.GetPlayerEffect(tp,EFFECT_NECRO_VALLEY)}
        let eff = duel::get_player_effect(d, tp, EFFECT_NECRO_VALLEY);
        // for _,te in ipairs(eff) do
        for te in eff {
            // if not op or op(e,c) then g:RemoveCard(c) break end
            if necro_blocks(d, te, e, c) {
                group::remove_card(&mut g, &CardSet::from_card(c));
                break;
            }
        }
    }
    // return g:CheckWithSumGreater(Card.GetLevel,8)
    group::check_with_sum_greater(d, &g, &|d, x| card::get_level(d, x) as i64, 8).0
}
/// The Necro Valley check of `s.spcon` and `s.spop`: `not op or op(e,c)` for an effect `te`.
fn necro_blocks(d: &mut Duel, te: EffectId, e: EffectId, c: CardId) -> bool {
    match effect::get_operation(d, te) {
        None => true,
        Some(f) => f.call(d, &[Value::Effect(e), Value::Card(c)]).truthy(),
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.int(1) as u8;
    let c = a.c(8);
    // local g=Duel.GetMatchingGroup(s.spfilter,c:GetControler(),LOCATION_HAND,0,nil)
    let cp = card::get_controler(d, c);
    let mut g = duel::get_matching_group(d, Some(&spfilter), cp, LOCATION_HAND, 0, NO_EX);
    // if not c:IsAbleToGraveAsCost() then g:RemoveCard(c) end
    if !card::is_able_to_grave_as_cost(d, c) {
        group::remove_card(&mut g, &CardSet::from_card(c));
    }
    // if g:IsContains(c) then
    if group::is_contains(&g, c) {
        // eff={Duel.GetPlayerEffect(tp,EFFECT_NECRO_VALLEY)}
        // (the Lua assigns the global `eff` here: nothing reads it, so it is not stored)
        let eff = duel::get_player_effect(d, tp, EFFECT_NECRO_VALLEY);
        // for _,te in ipairs(eff) do
        for te in eff {
            // if not op or op(e,c) then g:RemoveCard(c) break end
            if necro_blocks(d, te, e, c) {
                group::remove_card(&mut g, &CardSet::from_card(c));
                break;
            }
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sg=g:SelectWithSumGreater(tp,Card.GetLevel,8)
    let sg = group::select_with_sum_greater(d, &g, tp, &|d, x| card::get_level(d, x) as i64, 8).nil();
    // Duel.SendtoGrave(sg,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, sg, REASON_COST | REASON_DISCARD, None, None);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn destroy_op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    // if ep==tp then return end
    if a.ep == a.tp {
        return;
    }
    // if not re:IsActiveType(TYPE_EFFECT) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let re = a.re.nil();
    if !effect::is_active_type(d, re, TYPE_EFFECT) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // if g and g:IsContains(e:GetHandler()) then
    let contains = match info[0].as_group() {
        None => false,
        Some(g) => {
            let g = g.clone();
            let h = effect::get_handler(d, a.e).nil();
            group::is_contains(&g, h)
        }
    };
    if contains {
        // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let hg = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        // if #hg==0 then return end
        if hg.is_empty() {
            return;
        }
        // Duel.ConfirmCards(tp,hg)
        duel::confirm_cards(d, a.tp, hg.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
        // local sg=hg:Select(tp,1,1,nil)
        let sg = group::select(d, &hg, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
