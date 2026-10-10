//! Dogmatika Alba Zoa
use super::prelude::*;

pub const ID: u32 = 51522296;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={31002402}
    listed_names: &[31002402],
    // s.listed_series={SET_DOGMATIKA}
    listed_series: &[SET_DOGMATIKA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be special summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.ritlimit)
    effect::set_value_fn(d, e1, lua_fn(ritlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Unaffected by activated effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DOGMATIKA))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DOGMATIKA as u16]))));
    // e2:SetValue(s.unaval)
    effect::set_value_fn(d, e2, lua_fn(unaval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent applies 1 effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOEXTRA)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_TOEXTRA);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.applytg)
    effect::set_target(d, e3, Some(cost_target(applytg)));
    // e3:SetOperation(s.applyop)
    effect::set_operation(d, e3, Some(operation(applyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ritlimit(e,se,sp,st)`
fn ritlimit(d: &mut Duel, a: Args) -> bool {
    // if (st&SUMMON_TYPE_RITUAL)==SUMMON_TYPE_RITUAL then
    let st = a.int(3) as u32;
    if (st & SUMMON_TYPE_RITUAL) == SUMMON_TYPE_RITUAL {
        // return se:GetHandler():IsSetCard(SET_DOGMATIKA)
        let h = effect::get_handler(d, a.e(1)).nil();
        return card::is_set_card(d, h, &[SET_DOGMATIKA as u16]);
    }
    // return true
    true
}
/// `s.unaval(e,te)`
fn unaval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // local tc=te:GetOwner()
    let _tc = effect::get_owner(d, te);
    // return te:IsMonsterEffect() and te:IsActivated() and te:GetOwnerPlayer()==1-e:GetHandlerPlayer()
    //     and te:IsActiveType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)
    effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e)
        && effect::is_active_type(d, te, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK)
}
/// `s.applytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ex_ct=Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)
    let ex_ct = duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA);
    // local b1=ex_ct>=2 and Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,ex_ct//2,nil)
    let b1 = ex_ct >= 2 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND | LOCATION_EXTRA, (ex_ct / 2) as u32, NO_EX);
    // local texg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK),tp,0,LOCATION_MZONE,nil)
    let texg = duel::get_matching_group(d, Some(&faceup_link_type), tp, 0, LOCATION_MZONE, NO_EX);
    // local b2=#texg>0 and #texg==texg:FilterCount(Card.IsAbleToExtra,nil)
    let b2 = !texg.is_empty() && texg.len() == group::filter_count(d, &texg, &|d, c| card::is_able_to_extra(d, c), NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `aux.FaceupFilter(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)`
fn faceup_link_type(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK)
}
/// `s.applyop(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ex_ct=Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)
    let ex_ct = duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA);
    // local b1=ex_ct>=2 and Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,ex_ct//2,nil)
    let b1 = ex_ct >= 2 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND | LOCATION_EXTRA, (ex_ct / 2) as u32, NO_EX);
    // local texg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK),tp,0,LOCATION_MZONE,nil)
    let mut texg = duel::get_matching_group(d, Some(&faceup_link_type), tp, 0, LOCATION_MZONE, NO_EX);
    // local b2=#texg>0 and #texg==texg:FilterCount(Card.IsAbleToExtra,nil)
    let b2 = !texg.is_empty() && texg.len() == group::filter_count(d, &texg, &|d, c| card::is_able_to_extra(d, c), NO_EX);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(1-tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, 1 - tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // if op==1 then
    if op == 1 {
        // local send_ct=ex_ct//2
        let send_ct = ex_ct / 2;
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(1-tp,Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,send_ct,send_ct,nil)
        let g = duel::select_matching_card(d, 1 - tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND | LOCATION_EXTRA, send_ct as u16, send_ct as u16, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT,PLAYER_NONE,1-tp)
            duel::send_to_grave(d, g, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(1 - tp));
        }
    } else if op == 2 {
        // texg:Match(Card.IsAbleToExtra,nil)
        group::match_(d, &mut texg, &|d, c| card::is_able_to_extra(d, c), NO_EX);
        // Duel.SendtoDeck(texg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT,1-tp)
        duel::send_to_deck(d, texg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, Some(1 - tp));
    }
}
