//! Meklord Astro Dragon Triskelion
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 4837861;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must first be Special Summoned (from your hand) by banishing 3 "Meklord" monsters with different names from your GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Look at your opponent's Extra Deck and equip 1 monster from it to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card gains ATK equal to the combined ATK of those equipped monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,c) return e:GetHandler():GetEquipGroup():Filter(Card.HasFlagEffect,nil,id):GetSum(Card.GetAttack) end)
    effect::set_value_fn(d, e2, effect_value(atk_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While equipped with a Synchro Monster, this card can make up to 3 attacks on monsters during each Battle Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK_MONSTER);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():GetEquipGroup():IsExists(Card.IsOriginalType,1,nil,TYPE_SYNCHRO) end)
    effect::set_condition(d, e3, Some(condition(e3_condition)));
    // e3:SetValue(2)
    effect::set_value(d, e3, 2);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.listed_series={SET_MEKLORD}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEKLORD) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_MEKLORD as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:GetClassCount(Card.GetCode)==#sg,sg:GetClassCount(Card.GetCode)~=#sg
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len();
    let stop = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) != sg.len();
    (res, stop)
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
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return #rg>=2 and aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,0)
    let rs: aux::Rescon = &rescon;
    rg.len() >= 2
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
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
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:CheckUniqueOnField(tp) and c:IsMonster() and not c:IsForbidden()
    card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,0,LOCATION_EXTRA,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local g=Duel.GetMatchingGroup(s.eqfilter,tp,0,LOCATION_EXTRA,nil,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, tp)), tp, 0, LOCATION_EXTRA, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.ConfirmCards(tp,Duel.GetFieldGroup(tp,0,LOCATION_EXTRA))
        let fg = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
        duel::confirm_cards(d, tp, fg);
        // if c:IsFacedown() then return Duel.ShuffleExtra(1-tp) end
        if card::is_facedown(d, c) {
            duel::shuffle_extra(d, 1 - tp);
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &sel);
        // Duel.ShuffleExtra(1-tp)
        duel::shuffle_extra(d, 1 - tp);
        // if ec and Duel.Equip(tp,ec,c,true) then
        if let Some(ec) = ec {
            if duel::equip(d, tp, ec, c, true, false) {
                // ec:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
                card::register_flag_effect(d, ec, ID, RESET_EVENT + RESETS_STANDARD, 0, 1, 0, 0);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetValue(function(e,_c) return _c==c end)
                effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, cc| cc == c));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
            }
        }
    }
}
/// `function(e) return e:GetHandler():GetEquipGroup():IsExists(Card.IsOriginalType,1,nil,TYPE_SYNCHRO) end`
fn e3_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    group::is_exists(d, &eq, &|d, c| card::is_original_type(d, c, TYPE_SYNCHRO), 1, NO_EX)
}
/// `function(e,c) return e:GetHandler():GetEquipGroup():Filter(Card.HasFlagEffect,nil,id):GetSum(Card.GetAttack) end`
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let eq = card::get_equip_group(d, h);
    let g = group::filter(d, &eq, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64)
}
