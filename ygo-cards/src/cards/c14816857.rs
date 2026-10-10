//! Tri-Brigade Nervall
use super::prelude::*;

pub const ID: u32 = 14816857;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_TRI_BRIGADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon a Link Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Tribrigade" monster from Deck to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACES_BEAST_BWARRIOR_WINGB) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,false)
    card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64) && card::is_able_to_remove_as_cost(d, c, Some(POS_FACEUP as u8)) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spfilter(c,e,tp,ct,g)`: `g` is nil when the Lua passes nil
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ct: u64, g: Option<&CardSet>) -> bool {
    // return c:IsRace(RACES_BEAST_BWARRIOR_WINGB) and c:IsType(TYPE_LINK) and c:IsLink(ct)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    //     and Duel.GetLocationCountFromEx(tp,tp,g,c)>0
    if !(card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64) && card::is_type(d, c, TYPE_LINK) && card::is_link(d, c, &[ct as u32])) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None) {
        return false;
    }
    let ex = match g {
        Some(g) => g.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local nums={}
    // for i=1,#g do
    //     if Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,i,g) then table.insert(nums,i) end
    // end
    let mut nums: Vec<u64> = Vec::new();
    for i in 1..=g.len() {
        let gg = g.clone();
        if duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, i as u64, Some(&gg))), tp, LOCATION_EXTRA, 0, 1, NO_EX) {
            nums.push(i as u64);
        }
    }
    // if chk==0 then return #nums>0 end
    if a.chk == Some(0) {
        return !nums.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LVRANK as u64);
    // local ct=Duel.AnnounceNumber(tp,table.unpack(nums))
    let ct = duel::announce_number(d, tp, &nums).nil().0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=g:Select(tp,ct,ct,nil)
    let rg = group::select(d, &g, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct as i64]);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end --existence of card to summon checked in cost
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.matlimit(e,c)`
fn matlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
    !card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return c:IsControler(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    card::is_controler(d, c, p)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // register material limitation
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetTarget(s.matlimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| matlimit(d, e, c))));
    // e1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e1, LOCATION_ALL, LOCATION_ALL);
    // e1:SetValue(s.sumlimit)
    effect::set_value_fn(d, e1, lua_fn(sumlimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // client hint
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // special Summon
    // local ct=e:GetLabel()
    // if not ct then return end
    let ct = effect::get_label(d, e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,ct)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, ct as u64, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
