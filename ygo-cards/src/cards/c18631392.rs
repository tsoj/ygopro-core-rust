//! Ma'at
use super::prelude::*;

pub const ID: u32 = 18631392;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --announce 3 cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.anctg)
    effect::set_target(d, e3, Some(cost_target(anctg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let (chkf, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (chkf && group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsRace(RACE_FAIRY) and sg:FilterCount(Card.IsRace,c,RACE_DRAGON)==1
    card::is_race(d, c, RACE_FAIRY as u64) && group::filter_count(d, sg, &|d, x| card::is_race(d, x, RACE_DRAGON as u64), c.into()) == 1
}
/// `s.spfilter(c,rac)`
fn spfilter(d: &mut Duel, c: CardId, rac: u64) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(rac) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, rac) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,RACE_FAIRY)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_FAIRY as u64)), tp, LOCATION_MZONE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,RACE_DRAGON)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_DRAGON as u64)), tp, LOCATION_MZONE, 0, NO_EX);
    // local rg=rg1:Clone()
    // rg:Merge(rg2)
    let mut rg = rg1.clone();
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg1>0 and #rg2>0
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !rg1.is_empty()
        && !rg2.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,RACE_FAIRY|RACE_DRAGON)
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, (RACE_FAIRY | RACE_DRAGON) as u64)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    }
}
/// `s.anctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn anctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsPlayerCanDiscardDeck(tp,3) then return false end
        if !duel::is_player_can_discard_deck(d, tp, 3) {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,3)
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        let g = duel::get_decktop_group(d, tp, 3);
        return group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
    }
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    let announce_filter = [TYPE_EXTRA as u64, OPCODE_ISTYPE as u64, OPCODE_NOT as u64];
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // local ac1=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac1 = duel::announce_card(d, tp, &announce_filter) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // local ac2=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac2 = duel::announce_card(d, tp, &announce_filter) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // local ac3=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac3 = duel::announce_card(d, tp, &announce_filter) as u32;
    // e:SetOperation(s.retop(ac1,ac2,ac3))
    effect::set_operation(d, a.e, Some(operation(move |d, a| retop(d, a, ac1, ac2, ac3))));
    true
}
/// `s.hfilter(c,code1,code2,code3)`
fn hfilter(d: &mut Duel, c: CardId, code1: u32, code2: u32, code3: u32) -> bool {
    // return c:IsCode(code1,code2,code3) and c:IsAbleToHand()
    card::is_code(d, c, &[code1, code2, code3]) && card::is_able_to_hand(d, c, None)
}
/// `s.retop(code1,code2,code3)`: the returned `function(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs, code1: u32, code2: u32, code3: u32) {
    let tp = a.tp;
    // if not Duel.IsPlayerCanDiscardDeck(tp,3) then return end
    if !duel::is_player_can_discard_deck(d, tp, 3) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, tp, 3);
    // local g=Duel.GetDecktopGroup(tp,3)
    let mut g = duel::get_decktop_group(d, tp, 3);
    // local hg=g:Filter(s.hfilter,nil,code1,code2,code3)
    let hg = group::filter(d, &g, &|d, x| hfilter(d, x, code1, code2, code3), NO_EX);
    // g:Sub(hg)
    group::remove_card(&mut g, &hg);
    // if #hg~=0 then
    if !hg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(#hg*1000)
        effect::set_value(d, e1, hg.len() as i64 * 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
