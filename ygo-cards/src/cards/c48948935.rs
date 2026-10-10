//! Masked Beast Des Gardius
use super::prelude::*;

pub const ID: u32 = 48948935;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[22610082],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(13676474,86569121) and (c:IsControler(tp) or c:IsFaceup())
    card::is_code(d, c, &[13676474, 86569121]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg1=Duel.GetReleaseGroup(tp)
    let rg1 = duel::get_release_group(d, tp, false, false, None);
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,tp)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    let r = aux::chkf_mmz(1);
    // return aux.SelectUnselectGroup(rg1,e,tp,2,2,aux.ChkfMMZ(1),0)
    //     and aux.SelectUnselectGroup(rg2,e,tp,1,1,aux.ChkfMMZ(1),0)
    matches!(aux::select_unselect_group(d, &rg1, Some(e), tp, Some(2), Some(2), Some(&r), 0, tp, None, None, None, false), Err(true))
        && matches!(aux::select_unselect_group(d, &rg2, Some(e), tp, Some(1), Some(1), Some(&r), 0, tp, None, None, None, false), Err(true))
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,tp)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    let r = aux::chkf_mmz(1);
    // local mg1=aux.SelectUnselectGroup(rg1,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &rg1, Some(e), tp, Some(1), Some(1), Some(&r), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
        .unwrap_or_default();
    // if #mg1>0 then
    if !mg1.is_empty() {
        // local sg=mg1:GetFirst()
        let sg = group::get_first(d, &mg1).nil();
        // local rg2=Duel.GetReleaseGroup(tp)
        let mut rg2 = duel::get_release_group(d, tp, false, false, None);
        // rg2:RemoveCard(sg)
        group::remove_card(&mut rg2, &CardSet::from_card(sg));
        // local mg2=aux.SelectUnselectGroup(rg2,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &rg2, Some(e), tp, Some(1), Some(1), Some(&r), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
            .unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
    }
    // if #mg1==2 then
    if mg1.len() == 2 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, &g, REASON_COST, None);
    // g:DeleteGroup()  (groups are values: nothing to do)
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(22610082) and not c:IsForbidden()
    card::is_code(d, c, &[22610082]) && !card::is_forbidden(d, c)
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local eqc=g:GetFirst()
        // if not eqc or not Duel.Equip(tp,eqc,tc,true) then return end
        let Some(eqc) = group::get_first(d, &g) else {
            return;
        };
        if !duel::equip(d, tp, eqc, tc, true, false) {
            return;
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // eqc:RegisterEffect(e1)
        card::register_effect(d, eqc, e1, false);
        // local e2=Effect.CreateEffect(eqc)
        let e2 = effect::create_effect(d, eqc);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_SET_CONTROL)
        effect::set_code(d, e2, EFFECT_SET_CONTROL);
        // e2:SetValue(tp)
        effect::set_value(d, e2, tp as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // eqc:RegisterEffect(e2)
        card::register_effect(d, eqc, e2, false);
    }
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabelObject()==c
    effect::get_label_object(d, e).as_card() == Some(c)
}
