//! Moon of the Closed Heaven
use super::prelude::*;

pub const ID: u32 = 71818935;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // You can use 1 opponent's monster as Link Material with this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //     s.flagmap={}
    // end)
    aux::global_check(d, ID, |_d| {
        // s.flagmap={}: the lists of each card start empty (see flagmap)
    });
}
/// `s.flagmap[c]`: the flag effect this card's extra-material entry registered, kept in the duel
/// (key: the card, above the numbers the other script variables use).
fn flagmap_key(c: CardId) -> u64 {
    (1 << 32) | c.0 as u64
}
fn flagmap(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flagmap_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flagmap(d: &mut Duel, c: CardId, v: Vec<EffectId>) {
    set_s_var(d, ID, flagmap_key(c), Value::Multi(v.into_iter().map(Value::Effect).collect()));
}
/// `s.filter(c,lg)`
fn filter(d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return c:IsFaceup() and lg:IsContains(c)
    card::is_faceup(d, c) && lg.contains(c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, &lg);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,lg) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, &lg)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,lg)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, &lg)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // Can also use that opponent's monster as Link Material for a Link-5 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetAbsoluteRange(tp,1,0)
    effect::set_absolute_range(d, e1, a.tp, 1, 0);
    // e1:SetOperation(s.extracon)
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    // e1:SetValue(s.extraval)
    effect::set_value_fn(d, e1, lua_fn(extraval));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // c:CreateEffectRelation(e1)
    card::create_effect_relation(d, c, e1);
    // tc:SetCardTarget(c)
    card::set_card_target(d, tc, c);
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `function s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // local ct=sg:FilterCount(Card.HasFlagEffect,nil,e:GetFieldID())
    let fid = effect::get_field_id(d, e);
    let ct = group::get_count(&group::filter(d, &sg, &|d, c| card::get_flag_effect(d, c, fid) > 0, NO_EX));
    // return ct==0 or (sg+mg):Filter(s.extrafilter,nil,tp):IsExists(Card.IsRelateToEffect,1,og,e)
    ct == 0 || {
        let u = group::union(&sg, &mg);
        let f = group::filter(d, &u, &|d, c| extrafilter(d, c, tp), NO_EX);
        group::is_exists(d, &f, &|d, c| card::is_relate_to_effect(d, c, e), 1, (&og).into())
    }
}
/// `function s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let sc = a.card(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsLink(5) then
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_link(d, sc.nil(), &[5]) {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // s.flagmap[c]=c:RegisterFlagEffect(e:GetFieldID(),0,0,1)
        let fid = effect::get_field_id(d, e);
        let fe = card::register_flag_effect(d, c, fid, 0, 0, 1, 0, 0);
        set_flagmap(d, c, vec![fe]);
        // return Group.FromCards(c)
        return Value::Group(group::create_group(&[c]));
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
        }
    } else if chk == 2 {
        // if s.flagmap[c] then
        //     s.flagmap[c]:Reset()
        //     s.flagmap[c]=nil
        // end
        for eff in flagmap(d, c) {
            effect::reset(d, eff);
        }
        set_flagmap(d, c, Vec::new());
    }
    Value::Nil
}
